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
    ngx_flag_t    enable_report;              // for every location, we need flag to enable/disable mixer
    ngx_flag_t    enable_check;               // enable/disable check

} ngx_http_mixer_loc_conf_t;

/**
 * @brief element mixer configuration
 */
typedef struct {
    ngx_str_t mixer_server;              /**< mixer server */
    ngx_int_t mixer_port;                /**  mixer port */

} ngx_http_mixer_main_conf_t;

typedef struct {
    ngx_array_t   *key;             // array of label keys
    ngx_array_t   *value;           // array of label values
} service_labels_conf_t;

typedef struct  {
    ngx_str_t              destination_service;      // destination service
    ngx_str_t              destination_uid;          // destination service
    ngx_str_t              destination_ip;           // destination ip address
    service_labels_conf_t  destination_labels;       // destination labels
    ngx_str_t              source_ip;                // source ip
    ngx_str_t              source_uid;               // source uid
    ngx_str_t              source_service;           // source service
    ngx_uint_t             source_port;              // source port
    service_labels_conf_t  source_labels;            // source labels

} ngx_http_mixer_srv_conf_t;


char *save_service_labels_slot(ngx_conf_t *cf, ngx_command_t *cmd, void *conf);
char *service_labels_block(ngx_conf_t *cf, ngx_command_t *cmd, void *conf);

static ngx_int_t ngx_http_istio_mixer_report_handler(ngx_http_request_t *r);
static ngx_int_t ngx_http_istio_mixer_check_handler(ngx_http_request_t *r);

static ngx_int_t ngx_http_mixer_filter_init(ngx_conf_t *cf);

// create configuration
static void *ngx_http_mixer_create_loc_conf(ngx_conf_t *cf);
static char *ngx_http_mixer_merge_loc_conf(ngx_conf_t *cf, void *parent,void *child);

static void *ngx_http_mixer_create_srv_conf(ngx_conf_t *cf);
static char *ngx_http_mixer_merge_srv_conf(ngx_conf_t *cf, void *parent, void *child);

static void *ngx_http_mixer_create_main_conf(ngx_conf_t *cf);

// handlers in rust

void  nginmesh_mixer_report_handler(ngx_http_request_t *r, ngx_http_mixer_main_conf_t *main_conf,ngx_http_mixer_srv_conf_t *srv_conf);
ngx_int_t nginmesh_mixer_check_handler(ngx_http_request_t *r, ngx_http_mixer_main_conf_t *main_conf, ngx_http_mixer_srv_conf_t *srv_conf);

ngx_int_t  nginmesh_mixer_init(ngx_cycle_t *cycle);
void  nginmesh_mixer_exit();



/**
 * This module provide callback to istio for http traffic
 *
 */
static ngx_command_t ngx_http_istio_mixer_commands[] = {

    {
      ngx_string("mixer_report"),   /* report directive */
      NGX_HTTP_LOC_CONF | NGX_CONF_FLAG,
      ngx_conf_set_flag_slot, /* configuration setup function */
      NGX_HTTP_LOC_CONF_OFFSET,
      offsetof(ngx_http_mixer_loc_conf_t, enable_report),  // store in the location configuration
      NULL
    },
    {
       ngx_string("mixer_check"), /* directive */
       NGX_HTTP_LOC_CONF | NGX_CONF_FLAG,
       ngx_conf_set_flag_slot, /* configuration setup function */
       NGX_HTTP_LOC_CONF_OFFSET,
       offsetof(ngx_http_mixer_loc_conf_t, enable_check),  // store in the location configuration
       NULL
    },
    {
       ngx_string("mixer_destination_service"), /* directive */
       NGX_HTTP_SRV_CONF | NGX_CONF_TAKE1,
       ngx_conf_set_str_slot, /* configuration setup function */
       NGX_HTTP_SRV_CONF_OFFSET,
       offsetof(ngx_http_mixer_srv_conf_t, destination_service),  // store in the location configuration
       NULL
     },
     {
        ngx_string("mixer_destination_uid"), /* directive */
        NGX_HTTP_SRV_CONF | NGX_CONF_TAKE1,
        ngx_conf_set_str_slot, /* configuration setup function */
        NGX_HTTP_SRV_CONF_OFFSET,
        offsetof(ngx_http_mixer_srv_conf_t, destination_uid),  // store in the location configuration
        NULL
     },
     {
      ngx_string("mixer_destination_ip"), /* directive */
      NGX_HTTP_SRV_CONF | NGX_CONF_TAKE1,
      ngx_conf_set_str_slot, /* configuration setup function */
      NGX_HTTP_SRV_CONF_OFFSET,
      offsetof(ngx_http_mixer_srv_conf_t, destination_ip),  // store in the location configuration
      NULL
    },
    {
      ngx_string("mixer_destination_labels"),
      NGX_HTTP_SRV_CONF | NGX_CONF_BLOCK | NGX_CONF_NOARGS,
      service_labels_block,
      NGX_HTTP_SRV_CONF_OFFSET,
      offsetof(ngx_http_mixer_srv_conf_t, destination_labels),
      NULL
    },
    {
      ngx_string("mixer_source_ip"),
      NGX_HTTP_SRV_CONF | NGX_CONF_TAKE1,
      ngx_conf_set_str_slot,
      NGX_HTTP_SRV_CONF_OFFSET,
      offsetof(ngx_http_mixer_srv_conf_t, source_ip),  // store in the location configuration
      NULL
    },
    {
      ngx_string("mixer_source_uid"),
      NGX_HTTP_SRV_CONF | NGX_CONF_TAKE1,
      ngx_conf_set_str_slot,
      NGX_HTTP_SRV_CONF_OFFSET,
      offsetof(ngx_http_mixer_srv_conf_t, source_uid),  // store in the location configuration
      NULL
    },
    {
      ngx_string("mixer_source_labels"),
      NGX_HTTP_SRV_CONF | NGX_CONF_BLOCK | NGX_CONF_NOARGS,
      service_labels_block,
      NGX_HTTP_SRV_CONF_OFFSET,
      offsetof(ngx_http_mixer_srv_conf_t, source_labels),
      NULL
    },
    {
      ngx_string("mixer_source_service"),
      NGX_HTTP_SRV_CONF | NGX_CONF_TAKE1,
      ngx_conf_set_str_slot,
      NGX_HTTP_SRV_CONF_OFFSET,
      offsetof(ngx_http_mixer_srv_conf_t, source_service),  // store in the location configuration
      NULL
    },
    {
      ngx_string("mixer_source_port"),
      NGX_HTTP_SRV_CONF | NGX_CONF_TAKE1,
      ngx_conf_set_num_slot,
      NGX_HTTP_SRV_CONF_OFFSET,
      offsetof(ngx_http_mixer_srv_conf_t, source_port),  // store in the location configuration
      NULL
    },
    {
      ngx_string("mixer_server"), /* directive */
      NGX_HTTP_MAIN_CONF|NGX_CONF_TAKE1,  // server takes 1 //
      ngx_conf_set_str_slot, /* configuration setup function */
      NGX_HTTP_MAIN_CONF_OFFSET,
      offsetof(ngx_http_mixer_main_conf_t,mixer_server),
      NULL
    },
     {
      ngx_string("mixer_port"), /* directive */
      NGX_HTTP_MAIN_CONF|NGX_CONF_TAKE1, // server port takes 1 //
      ngx_conf_set_num_slot, /* configuration setup function */
      NGX_HTTP_MAIN_CONF_OFFSET,
      offsetof(ngx_http_mixer_main_conf_t,mixer_port),
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

    ngx_http_mixer_create_srv_conf, /* create server configuration */
    ngx_http_mixer_merge_srv_conf, /* merge server configuration */

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
    nginmesh_mixer_init, /* init process */
    NULL, /* init thread */
    NULL, /* exit thread */
    NULL, /* exit process */
    NULL, /* exit master */
    NGX_MODULE_V1_PADDING
};

char *service_labels_block(ngx_conf_t *cf, ngx_command_t *cmd, void *conf)
{
    char                  *rv;
    ngx_conf_t            save;
    service_labels_conf_t *ctx;

    // Get configuration context - Srv in this case.
    char  *p = conf;
    // Add offset of particular label value within structure
    ctx = (service_labels_conf_t *) (p + cmd->offset);

    ngx_log_debug(NGX_LOG_DEBUG_HTTP, ngx_cycle->log, 0, "Entering Service Labels Block~");

    // Save old configuration
    save = *cf;

    // Set handler for each line of config block to custom handler
    // Reference context within custom struct and current config context
    cf->ctx = ctx;
    cf->handler = save_service_labels_slot;
    cf->handler_conf = conf;

    // Parse block
    rv = ngx_conf_parse(cf, NULL);

    // Restore configuration
    *cf = save;

    if (rv != NGX_CONF_OK) {
        return rv;
    }

    return NGX_CONF_OK;
}

char *save_service_labels_slot(ngx_conf_t *cf, ngx_command_t *cmd, void *conf) {

    ngx_str_t                *value, *c;
    service_labels_conf_t    *ctx;

    // Get context set in block
    ctx = cf->ctx;

    ngx_log_debug(NGX_LOG_DEBUG_EVENT, ngx_cycle->log, 0, "Entering Service Labels Save");

    // Get single line's arguments
    value = cf->args->elts;

    // If elements don't take form of key/value pairs, error
    if (cf->args->nelts != 2){
        ngx_conf_log_error(NGX_LOG_EMERG, cf, 0,
                            "Labels must be in form of key value pairs");
        return NGX_CONF_ERROR;
    }

    // If key array is NULL, create new array
    if (ctx->key == NULL) {
        ctx->key = ngx_array_create(cf->pool, 4, sizeof(ngx_str_t));
        if (ctx->key == NULL) {
            return NGX_CONF_ERROR;
        }
    }
    // If value array is NULL, create new array
    if (ctx->value == NULL) {
        ctx->value = ngx_array_create(cf->pool, 4, sizeof(ngx_str_t));
        if (ctx->value == NULL) {
            return NGX_CONF_ERROR;
        }
    }

    // Extract key/value pair
    c = ngx_array_push(ctx->key);
    if (c == NULL) {
        return NGX_CONF_ERROR;
    }
    *c = value[0];

    c = ngx_array_push(ctx->value);
    if (c == NULL) {
        return NGX_CONF_ERROR;
    }
    *c = value[1];

    return NGX_CONF_OK;

}

// install log phase handler for mixer
static ngx_int_t ngx_http_mixer_filter_init(ngx_conf_t *cf) {


    ngx_http_handler_pt        *h1,*h2;
    ngx_http_core_main_conf_t  *cmcf;
    ngx_http_core_loc_conf_t  *clcf;

    cmcf = ngx_http_conf_get_module_main_conf(cf, ngx_http_core_module);

    h1 = ngx_array_push(&cmcf->phases[NGX_HTTP_LOG_PHASE].handlers);
    if (h1 == NULL) {
        return NGX_ERROR;
    }
    *h1 = ngx_http_istio_mixer_report_handler;

    ngx_log_debug(NGX_LOG_DEBUG_EVENT, ngx_cycle->log, 0, "registering mixer_report handler");

    clcf = ngx_http_conf_get_module_loc_conf(cf, ngx_http_core_module);
    clcf->handler = ngx_http_istio_mixer_check_handler;

    h2 = ngx_array_push(&cmcf->phases[NGX_HTTP_ACCESS_PHASE].handlers);
    if (h2 == NULL) {
        return NGX_ERROR;
    }
    *h2 = ngx_http_istio_mixer_check_handler;

    ngx_log_debug(NGX_LOG_DEBUG_EVENT, ngx_cycle->log, 0, "registering mixer_check handler");




    return NGX_OK;
}

/**
 * mixer report handler.
 *
 */
static ngx_int_t ngx_http_istio_mixer_report_handler(ngx_http_request_t *r)
{
    ngx_http_mixer_loc_conf_t  *loc_conf;
    ngx_http_mixer_main_conf_t *main_conf;
    ngx_http_mixer_srv_conf_t *srv_conf;

    ngx_log_debug(NGX_LOG_DEBUG_HTTP,  r->connection->log, 0, "start invoking istio mixer report handler");

    loc_conf = ngx_http_get_module_loc_conf(r, ngx_http_istio_mixer_module);

    if (!loc_conf->enable_report) {
        ngx_log_debug(NGX_LOG_DEBUG_HTTP,  r->connection->log, 0, "mixer_report not enabled, just passing thru");
        return NGX_OK;
    }

    srv_conf = ngx_http_get_module_srv_conf(r,ngx_http_istio_mixer_module);


    main_conf = ngx_http_get_module_main_conf(r, ngx_http_istio_mixer_module);

    ngx_log_debug2(NGX_LOG_DEBUG_HTTP,  r->connection->log, 0, "using mixer server: %*s",main_conf->mixer_server.len,main_conf->mixer_server.data);

    // invoke mix client
    nginmesh_mixer_report_handler(r,main_conf,srv_conf);

    ngx_log_debug(NGX_LOG_DEBUG_HTTP,  r->connection->log, 0, "finish calling mixer report handler");


   return NGX_OK;

}


/**
 * check handler. this works in pre-access phase
 */
static ngx_int_t ngx_http_istio_mixer_check_handler(ngx_http_request_t *r)
{
    ngx_http_mixer_loc_conf_t  *loc_conf;
    ngx_http_mixer_main_conf_t *main_conf;
    ngx_http_mixer_srv_conf_t *srv_conf;

    ngx_log_debug(NGX_LOG_DEBUG_HTTP,  r->connection->log, 0, "start invoking mixer_check handler");

    loc_conf = ngx_http_get_module_loc_conf(r, ngx_http_istio_mixer_module);


    if (!loc_conf->enable_check) {
        ngx_log_debug(NGX_LOG_DEBUG_HTTP,  r->connection->log, 0, "mixer check not enabled, passing thru");
        return NGX_OK;
    }


    srv_conf = ngx_http_get_module_srv_conf(r,ngx_http_istio_mixer_module);

    main_conf = ngx_http_get_module_main_conf(r, ngx_http_istio_mixer_module);

    return nginmesh_mixer_check_handler(r,main_conf,srv_conf);


}


static void *ngx_http_mixer_create_loc_conf(ngx_conf_t *cf) {

    ngx_http_mixer_loc_conf_t  *conf;

    conf = ngx_pcalloc(cf->pool, sizeof(ngx_http_mixer_loc_conf_t));
    if (conf == NULL) {
        return NULL;
    }

    conf->enable_report = NGX_CONF_UNSET;
    conf->enable_check = NGX_CONF_UNSET;


    ngx_log_debug(NGX_LOG_DEBUG_EVENT, ngx_cycle->log, 0, "set up  mixer location config");

    return conf;
}

static char *ngx_http_mixer_merge_loc_conf(ngx_conf_t *cf, void *parent, void *child)
{
    ngx_log_debug(NGX_LOG_DEBUG_EVENT, ngx_cycle->log, 0, "merging loc conf");

    ngx_http_mixer_loc_conf_t  *prev = parent;
    ngx_http_mixer_loc_conf_t  *conf = child;


    ngx_conf_merge_value(conf->enable_report, prev->enable_report, 0);
    ngx_conf_merge_value(conf->enable_check, prev->enable_check, 0);


    return NGX_CONF_OK;
}

static void *ngx_http_mixer_create_srv_conf(ngx_conf_t *cf) {

    ngx_http_mixer_srv_conf_t  *conf;

    conf = ngx_pcalloc(cf->pool, sizeof(ngx_http_mixer_srv_conf_t));
    if (conf == NULL) {
        return NULL;
    }

    conf->source_port = NGX_CONF_UNSET_UINT;

    ngx_log_debug(NGX_LOG_DEBUG_EVENT, ngx_cycle->log, 0, "set up  mixer srv config");

    return conf;
}


static char *ngx_http_mixer_merge_srv_conf(ngx_conf_t *cf, void *parent, void *child)
{
    ngx_log_debug(NGX_LOG_DEBUG_EVENT, ngx_cycle->log, 0, "merging srv conf");

    ngx_http_mixer_srv_conf_t  *prev = parent;
    ngx_http_mixer_srv_conf_t  *conf = child;


    ngx_conf_merge_str_value(conf->destination_service,prev->destination_service,"");
    ngx_conf_merge_str_value(conf->destination_uid,prev->destination_uid,"");
    ngx_conf_merge_ptr_value(conf->destination_labels.key,prev->destination_labels.key,NULL);
    ngx_conf_merge_str_value(conf->source_ip,prev->source_ip,"");
    ngx_conf_merge_str_value(conf->source_uid,prev->source_uid,"");
    ngx_conf_merge_str_value(conf->source_service,prev->source_service,"");
    ngx_conf_merge_uint_value(conf->source_port, prev->source_port, 0);
    ngx_conf_merge_ptr_value(conf->destination_labels.value,prev->destination_labels.value,NULL);

    return NGX_CONF_OK;
}




static void *ngx_http_mixer_create_main_conf(ngx_conf_t *cf)
{
  ngx_http_mixer_main_conf_t *conf;

  ngx_log_debug(NGX_LOG_DEBUG_EVENT, ngx_cycle->log, 0, "setting up main config");


  conf = ngx_pcalloc(cf->pool, sizeof(ngx_http_mixer_main_conf_t));
  if (conf == NULL) {
    return NULL;
  }

  conf->mixer_port = NGX_CONF_UNSET_UINT;


  return conf;
}