#include <stdint.h>
#include <stddef.h>
#include <xf86drmMode.h>

const unsigned int MACRO_DRM_MODE_PROP_EXTENDED_TYPE    = DRM_MODE_PROP_EXTENDED_TYPE;
const unsigned int MACRO_DRM_MODE_PROP_OBJECT           = DRM_MODE_PROP_OBJECT;
const unsigned int MACRO_DRM_MODE_PROP_SIGNED_RANGE     = DRM_MODE_PROP_SIGNED_RANGE;

const unsigned int MACRO_DRM_MODE_ATOMIC_ALLOW_MODESET  = DRM_MODE_ATOMIC_ALLOW_MODESET;

const long MACRO_DRM_IOCTL_VERSION          = DRM_IOWR(0x00, struct drm_version);
const long MACRO_DRM_IOCTL_GET_UNIQUE       = DRM_IOWR(0x01, struct drm_unique);
const long MACRO_DRM_IOCTL_GET_MAGIC        = DRM_IOR( 0x02, struct drm_auth);
const long MACRO_DRM_IOCTL_IRQ_BUSID        = DRM_IOWR(0x03, struct drm_irq_busid);
const long MACRO_DRM_IOCTL_GET_MAP          = DRM_IOWR(0x04, struct drm_map);
const long MACRO_DRM_IOCTL_GET_CLIENT       = DRM_IOWR(0x05, struct drm_client);
const long MACRO_DRM_IOCTL_GET_STATS        = DRM_IOR( 0x06, struct drm_stats);
const long MACRO_DRM_IOCTL_SET_VERSION      = DRM_IOWR(0x07, struct drm_set_version);
const long MACRO_DRM_IOCTL_MODESET_CTL      = DRM_IOW(0x08, struct drm_modeset_ctl);
const long MACRO_DRM_IOCTL_GEM_CLOSE        = DRM_IOW (0x09, struct drm_gem_close);
const long MACRO_DRM_IOCTL_GEM_FLINK        = DRM_IOWR(0x0a, struct drm_gem_flink);
const long MACRO_DRM_IOCTL_GEM_OPEN         = DRM_IOWR(0x0b, struct drm_gem_open);
const long MACRO_DRM_IOCTL_GET_CAP          = DRM_IOWR(0x0c, struct drm_get_cap);
const long MACRO_DRM_IOCTL_SET_CLIENT_CAP   = DRM_IOW( 0x0d, struct drm_set_client_cap);

const long MACRO_DRM_IOCTL_SET_UNIQUE       = DRM_IOW( 0x10, struct drm_unique);
const long MACRO_DRM_IOCTL_AUTH_MAGIC       = DRM_IOW( 0x11, struct drm_auth);
const long MACRO_DRM_IOCTL_BLOCK            = DRM_IOWR(0x12, struct drm_block);
const long MACRO_DRM_IOCTL_UNBLOCK          = DRM_IOWR(0x13, struct drm_block);
const long MACRO_DRM_IOCTL_CONTROL          = DRM_IOW( 0x14, struct drm_control);
const long MACRO_DRM_IOCTL_ADD_MAP          = DRM_IOWR(0x15, struct drm_map);
const long MACRO_DRM_IOCTL_ADD_BUFS         = DRM_IOWR(0x16, struct drm_buf_desc);
const long MACRO_DRM_IOCTL_MARK_BUFS        = DRM_IOW( 0x17, struct drm_buf_desc);
const long MACRO_DRM_IOCTL_INFO_BUFS        = DRM_IOWR(0x18, struct drm_buf_info);
const long MACRO_DRM_IOCTL_MAP_BUFS         = DRM_IOWR(0x19, struct drm_buf_map);
const long MACRO_DRM_IOCTL_FREE_BUFS        = DRM_IOW( 0x1a, struct drm_buf_free);

const long MACRO_DRM_IOCTL_RM_MAP           = DRM_IOW( 0x1b, struct drm_map);

const long MACRO_DRM_IOCTL_SET_SAREA_CTX    = DRM_IOW( 0x1c, struct drm_ctx_priv_map);
const long MACRO_DRM_IOCTL_GET_SAREA_CTX    = DRM_IOWR(0x1d, struct drm_ctx_priv_map);

const long MACRO_DRM_IOCTL_SET_MASTER       = DRM_IO(0x1e);
const long MACRO_DRM_IOCTL_DROP_MASTER      = DRM_IO(0x1f);

const long MACRO_DRM_IOCTL_ADD_CTX      = DRM_IOWR(0x20, struct drm_ctx);
const long MACRO_DRM_IOCTL_RM_CTX       = DRM_IOWR(0x21, struct drm_ctx);
const long MACRO_DRM_IOCTL_MOD_CTX      = DRM_IOW( 0x22, struct drm_ctx);
const long MACRO_DRM_IOCTL_GET_CTX      = DRM_IOWR(0x23, struct drm_ctx);
const long MACRO_DRM_IOCTL_SWITCH_CTX   = DRM_IOW( 0x24, struct drm_ctx);
const long MACRO_DRM_IOCTL_NEW_CTX      = DRM_IOW( 0x25, struct drm_ctx);
const long MACRO_DRM_IOCTL_RES_CTX      = DRM_IOWR(0x26, struct drm_ctx_res);
const long MACRO_DRM_IOCTL_ADD_DRAW     = DRM_IOWR(0x27, struct drm_draw);
const long MACRO_DRM_IOCTL_RM_DRAW      = DRM_IOWR(0x28, struct drm_draw);
const long MACRO_DRM_IOCTL_DMA          = DRM_IOWR(0x29, struct drm_dma);
const long MACRO_DRM_IOCTL_LOCK         = DRM_IOW( 0x2a, struct drm_lock);
const long MACRO_DRM_IOCTL_UNLOCK       = DRM_IOW( 0x2b, struct drm_lock);
const long MACRO_DRM_IOCTL_FINISH       = DRM_IOW( 0x2c, struct drm_lock);

const long MACRO_DRM_IOCTL_PRIME_HANDLE_TO_FD     = DRM_IOWR(0x2d, struct drm_prime_handle);
const long MACRO_DRM_IOCTL_PRIME_FD_TO_HANDLE     = DRM_IOWR(0x2e, struct drm_prime_handle);

const long MACRO_DRM_IOCTL_AGP_ACQUIRE  = DRM_IO(  0x30);
const long MACRO_DRM_IOCTL_AGP_RELEASE  = DRM_IO(  0x31);
const long MACRO_DRM_IOCTL_AGP_ENABLE   = DRM_IOW( 0x32, struct drm_agp_mode);
const long MACRO_DRM_IOCTL_AGP_INFO     = DRM_IOR( 0x33, struct drm_agp_info);
const long MACRO_DRM_IOCTL_AGP_ALLOC    = DRM_IOWR(0x34, struct drm_agp_buffer);
const long MACRO_DRM_IOCTL_AGP_FREE     = DRM_IOW( 0x35, struct drm_agp_buffer);
const long MACRO_DRM_IOCTL_AGP_BIND     = DRM_IOW( 0x36, struct drm_agp_binding);
const long MACRO_DRM_IOCTL_AGP_UNBIND   = DRM_IOW( 0x37, struct drm_agp_binding);

const long MACRO_DRM_IOCTL_SG_ALLOC     = DRM_IOWR(0x38, struct drm_scatter_gather);
const long MACRO_DRM_IOCTL_SG_FREE      = DRM_IOW( 0x39, struct drm_scatter_gather);

const long MACRO_DRM_IOCTL_WAIT_VBLANK        = DRM_IOWR(0x3a, union drm_wait_vblank);

const long MACRO_DRM_IOCTL_UPDATE_DRAW        = DRM_IOW(0x3f, struct drm_update_draw);

const long MACRO_DRM_IOCTL_MODE_GETRESOURCES    = DRM_IOWR(0xA0, struct drm_mode_card_res);
const long MACRO_DRM_IOCTL_MODE_GETCRTC         = DRM_IOWR(0xA1, struct drm_mode_crtc);
const long MACRO_DRM_IOCTL_MODE_SETCRTC         = DRM_IOWR(0xA2, struct drm_mode_crtc);
const long MACRO_DRM_IOCTL_MODE_CURSOR          = DRM_IOWR(0xA3, struct drm_mode_cursor);
const long MACRO_DRM_IOCTL_MODE_GETGAMMA        = DRM_IOWR(0xA4, struct drm_mode_crtc_lut);
const long MACRO_DRM_IOCTL_MODE_SETGAMMA        = DRM_IOWR(0xA5, struct drm_mode_crtc_lut);
const long MACRO_DRM_IOCTL_MODE_GETENCODER      = DRM_IOWR(0xA6, struct drm_mode_get_encoder);
const long MACRO_DRM_IOCTL_MODE_GETCONNECTOR    = DRM_IOWR(0xA7, struct drm_mode_get_connector);
const long MACRO_DRM_IOCTL_MODE_ATTACHMODE      = DRM_IOWR(0xA8, struct drm_mode_mode_cmd); /* deprecated (never worked) */
const long MACRO_DRM_IOCTL_MODE_DETACHMODE      = DRM_IOWR(0xA9, struct drm_mode_mode_cmd); /* deprecated (never worked) */

const long MACRO_DRM_IOCTL_MODE_GETPROPERTY     = DRM_IOWR(0xAA, struct drm_mode_get_property);
const long MACRO_DRM_IOCTL_MODE_SETPROPERTY     = DRM_IOWR(0xAB, struct drm_mode_connector_set_property);
const long MACRO_DRM_IOCTL_MODE_GETPROPBLOB     = DRM_IOWR(0xAC, struct drm_mode_get_blob);
const long MACRO_DRM_IOCTL_MODE_GETFB           = DRM_IOWR(0xAD, struct drm_mode_fb_cmd);
const long MACRO_DRM_IOCTL_MODE_ADDFB           = DRM_IOWR(0xAE, struct drm_mode_fb_cmd);
const long MACRO_DRM_IOCTL_MODE_RMFB            = DRM_IOWR(0xAF, unsigned int);
const long MACRO_DRM_IOCTL_MODE_PAGE_FLIP       = DRM_IOWR(0xB0, struct drm_mode_crtc_page_flip);
const long MACRO_DRM_IOCTL_MODE_DIRTYFB         = DRM_IOWR(0xB1, struct drm_mode_fb_dirty_cmd);

const long MACRO_DRM_IOCTL_MODE_CREATE_DUMB         = DRM_IOWR(0xB2, struct drm_mode_create_dumb);
const long MACRO_DRM_IOCTL_MODE_MAP_DUMB            = DRM_IOWR(0xB3, struct drm_mode_map_dumb);
const long MACRO_DRM_IOCTL_MODE_DESTROY_DUMB        = DRM_IOWR(0xB4, struct drm_mode_destroy_dumb);
const long MACRO_DRM_IOCTL_MODE_GETPLANERESOURCES   = DRM_IOWR(0xB5, struct drm_mode_get_plane_res);
const long MACRO_DRM_IOCTL_MODE_GETPLANE            = DRM_IOWR(0xB6, struct drm_mode_get_plane);
const long MACRO_DRM_IOCTL_MODE_SETPLANE            = DRM_IOWR(0xB7, struct drm_mode_set_plane);
const long MACRO_DRM_IOCTL_MODE_ADDFB2              = DRM_IOWR(0xB8, struct drm_mode_fb_cmd2);
const long MACRO_DRM_IOCTL_MODE_OBJ_GETPROPERTIES   = DRM_IOWR(0xB9, struct drm_mode_obj_get_properties);
const long MACRO_DRM_IOCTL_MODE_OBJ_SETPROPERTY     = DRM_IOWR(0xBA, struct drm_mode_obj_set_property);
const long MACRO_DRM_IOCTL_MODE_CURSOR2             = DRM_IOWR(0xBB, struct drm_mode_cursor2);
const long MACRO_DRM_IOCTL_MODE_ATOMIC              = DRM_IOWR(0xBC, struct drm_mode_atomic);
const long MACRO_DRM_IOCTL_MODE_CREATEPROPBLOB      = DRM_IOWR(0xBD, struct drm_mode_create_blob);
const long MACRO_DRM_IOCTL_MODE_DESTROYPROPBLOB     = DRM_IOWR(0xBE, struct drm_mode_destroy_blob);

