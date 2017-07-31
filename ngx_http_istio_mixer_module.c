/**
 * @file   ngx_http_istio_mixer_module.c
 * @author Sehyo Chang <sehyo@nginx.com>
 * @date   Wed Aug 19 2017
 *
 * @brief  Istio Mixer integration  module for Nginx.
 *
 * @section LICENSE
 *
 * Copyright (C) 2011 by Nginx
 *
 */
#include <ngx_config.h>
#include <ngx_core.h>
#include <ngx_http.h>


typedef struct {
    ngx_hash_t                          types;
    ngx_array_t                        *types_keys;
    ngx_http_complex_value_t           *variable;
} ngx_http_mixer_loc_conf_t;


/**
 * @brief element mixer configuration
 */
typedef struct {
    ngx_str_t              mixer_server;              /**< mixer server */
} ngx_http_mixer_main_conf_t;


static char *ngx_http_istio_mixer(ngx_conf_t *cf, ngx_command_t *cmd, void *conf);
static ngx_int_t ngx_http_istio_mixer_filter(ngx_http_request_t *r);
static ngx_int_t ngx_http_mixer_filter_init(ngx_conf_t *cf);

// create configuration
static void *ngx_http_mixer_create_loc_conf(ngx_conf_t *cf);
static char *ngx_http_mixer_merge_loc_conf(ngx_conf_t *cf, void *parent,
    void *child);

static void *ngx_http_mixer_create_main_conf(ngx_conf_t *cf);    


char  *mixer_client(ngx_http_request_t *r);

static ngx_http_output_header_filter_pt ngx_http_next_header_filter;


/**
 * This module provide callback to istio for http traffic
 *
 */
static ngx_command_t ngx_http_istio_mixer_commands[] = {

    { ngx_string("mixer"), /* directive */
      NGX_HTTP_LOC_CONF|NGX_CONF_NOARGS, /* location context and takes
                                            no arguments*/
      ngx_http_istio_mixer, /* configuration setup function */
      0, /* No offset. Only one context is supported. */
      0, /* No offset when storing the module configuration on struct. */
      NULL},

    { 
      ngx_string("mixer_server"), /* directive */
      NGX_HTTP_MAIN_CONF|NGX_CONF_TAKE1, /* location context and takes
                                            no arguments*/
      ngx_conf_set_str_slot, /* configuration setup function */
      NGX_HTTP_MAIN_CONF_OFFSET, 
      offsetof(ngx_http_mixer_main_conf_t,mixer_server),
      NULL
    },  

    ngx_null_command /* command termination */
};



/* The module context. */
static ngx_http_module_t ngx_http_istio_mixer_module_ctx = {
    NULL, /* preconfiguration */
    ngx_http_mixer_filter_init, /* postconfiguration */
    ngx_http_mixer_create_main_conf, /* create main configuration */
    NULL, /* init main configuration */

    NULL, /* create server configuration */
    NULL, /* merge server configuration */

    ngx_http_mixer_create_loc_conf, /* create location configuration */
    ngx_http_mixer_merge_loc_conf /* merge location configuration */
};

/* Module definition. */
ngx_module_t ngx_http_istio_mixer_module = {
    NGX_MODULE_V1,
    &ngx_http_istio_mixer_module_ctx, /* module context */
    ngx_http_istio_mixer_commands, /* module directives */
    NGX_HTTP_MODULE, /* module type */
    NULL, /* init master */
    NULL, /* init module */
    NULL, /* init process */
    NULL, /* init thread */
    NULL, /* exit thread */
    NULL, /* exit process */
    NULL, /* exit master */
    NGX_MODULE_V1_PADDING
};

static ngx_int_t ngx_http_mixer_filter_init(ngx_conf_t *cf) {

    
    ngx_http_next_header_filter = ngx_http_top_header_filter;
    ngx_http_top_header_filter = ngx_http_istio_mixer_filter;


    return NGX_OK;   
}

/**
 * Content handler.
 *
 * @param r
 *   Pointer to the request structure. See http_request.h.
 * @return
 *   The status of the response generation.
 */
static ngx_int_t ngx_http_istio_mixer_filter(ngx_http_request_t *r)
{

    ngx_log_error(NGX_LOG_ERR, ngx_cycle->log, 0, "start invoking istio mixer filter");

    ngx_http_mixer_main_conf_t *conf = ngx_http_get_module_main_conf(r, ngx_http_istio_mixer_module);

    ngx_log_error(NGX_LOG_ERR, ngx_cycle->log, 0, "using server: %*s",conf->mixer_server.len,conf->mixer_server.data);

    // invoke mix client
    mixer_client(r);

   ngx_log_error(NGX_LOG_ERR, ngx_cycle->log, 0, "finish calling istio filter");


   return ngx_http_next_header_filter(r);

} 


static void *ngx_http_mixer_create_loc_conf(ngx_conf_t *cf) {

    ngx_log_error(NGX_LOG_ERR, ngx_cycle->log, 0, "creating loc conf");

    ngx_http_mixer_loc_conf_t  *conf;

    conf = ngx_pcalloc(cf->pool, sizeof(ngx_http_mixer_loc_conf_t));
    if (conf == NULL) {
        return NULL;
    }

    /*
     * set by ngx_pcalloc():
     *
     *     conf->types = { NULL };
     *     conf->types_keys = NULL;
     *     conf->variable = NULL;
     */

    return conf;
}

static char *
ngx_http_mixer_merge_loc_conf(ngx_conf_t *cf, void *parent, void *child)
{
    ngx_log_error(NGX_LOG_ERR, ngx_cycle->log, 0, "merging loc conf");

    ngx_http_mixer_loc_conf_t  *prev = parent;
    ngx_http_mixer_loc_conf_t  *conf = child;

    if (ngx_http_merge_types(cf, &conf->types_keys, &conf->types,
                             &prev->types_keys,&prev->types,
                             ngx_http_html_default_types)
        != NGX_OK)
    {
       return NGX_CONF_ERROR;
    }

    if (conf->variable == NULL) {
        conf->variable = prev->variable;
    }

    if (conf->variable == NULL) {
        conf->variable = (ngx_http_complex_value_t *) -1;
    }

    return NGX_CONF_OK;
}

// init config
// borrow from https://github.com/alibaba/nginx-http-footer-filter/blob/master/ngx_http_footer_filter_module.c
// not sure what this code is doing...

static char *ngx_http_istio_mixer(ngx_conf_t *cf, ngx_command_t *cmd, void *conf)
{
   
     ngx_log_error(NGX_LOG_ERR, ngx_cycle->log, 0, "configuring mixer");

    return NGX_OK;
}


static void *ngx_http_mixer_create_main_conf(ngx_conf_t *cf)
{
  ngx_http_mixer_main_conf_t *conf;

  conf = ngx_pcalloc(cf->pool, sizeof(ngx_http_mixer_main_conf_t));
  if (conf == NULL) {
    return NULL;
  }


  return conf;
}