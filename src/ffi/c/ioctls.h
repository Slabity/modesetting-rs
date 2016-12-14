#include <libdrm/drm.h>
#include <libdrm/drm_mode.h>

const long MACRO_DRM_IOCTL_SET_MASTER       = DRM_IO(0x1e);
const long MACRO_DRM_IOCTL_DROP_MASTER      = DRM_IO(0x1f);
const long MACRO_DRM_IOCTL_SET_CLIENT_CAP   = DRM_IO(0x0d);

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

