#include <libdrm/drm.h>

/*
 * Calculate the ioctl numbers from the defines
 * First let's change the defines to a different name
 */
const unsigned long FFI_DRM_IOCTL_VERSION =          DRM_IOCTL_VERSION;
const unsigned long FFI_DRM_IOCTL_GET_UNIQUE =       DRM_IOCTL_GET_UNIQUE;
const unsigned long FFI_DRM_IOCTL_GET_MAGIC =        DRM_IOCTL_GET_MAGIC;
const unsigned long FFI_DRM_IOCTL_IRQ_BUSID =        DRM_IOCTL_IRQ_BUSID;
const unsigned long FFI_DRM_IOCTL_GET_MAP =          DRM_IOCTL_GET_MAP;
const unsigned long FFI_DRM_IOCTL_GET_CLIENT =       DRM_IOCTL_GET_CLIENT;
const unsigned long FFI_DRM_IOCTL_GET_STATS =        DRM_IOCTL_GET_STATS;
const unsigned long FFI_DRM_IOCTL_SET_VERSION =      DRM_IOCTL_SET_VERSION;
const unsigned long FFI_DRM_IOCTL_MODESET_CTL =      DRM_IOCTL_MODESET_CTL;
const unsigned long FFI_DRM_IOCTL_GEM_CLOSE =        DRM_IOCTL_GEM_CLOSE;
const unsigned long FFI_DRM_IOCTL_GEM_FLINK =        DRM_IOCTL_GEM_FLINK;
const unsigned long FFI_DRM_IOCTL_GEM_OPEN =         DRM_IOCTL_GEM_OPEN;
const unsigned long FFI_DRM_IOCTL_GET_CAP =          DRM_IOCTL_GET_CAP;
const unsigned long FFI_DRM_IOCTL_SET_CLIENT_CAP =   DRM_IOCTL_SET_CLIENT_CAP;

const unsigned long FFI_DRM_IOCTL_SET_UNIQUE =   DRM_IOCTL_SET_UNIQUE;
const unsigned long FFI_DRM_IOCTL_AUTH_MAGIC =   DRM_IOCTL_AUTH_MAGIC;
const unsigned long FFI_DRM_IOCTL_BLOCK =        DRM_IOCTL_BLOCK;
const unsigned long FFI_DRM_IOCTL_UNBLOCK =      DRM_IOCTL_UNBLOCK;
const unsigned long FFI_DRM_IOCTL_CONTROL =      DRM_IOCTL_CONTROL;
const unsigned long FFI_DRM_IOCTL_ADD_MAP =      DRM_IOCTL_ADD_MAP;
const unsigned long FFI_DRM_IOCTL_ADD_BUFS =     DRM_IOCTL_ADD_BUFS;
const unsigned long FFI_DRM_IOCTL_MARK_BUFS =    DRM_IOCTL_MARK_BUFS;
const unsigned long FFI_DRM_IOCTL_INFO_BUFS =    DRM_IOCTL_INFO_BUFS;
const unsigned long FFI_DRM_IOCTL_MAP_BUFS =     DRM_IOCTL_MAP_BUFS;
const unsigned long FFI_DRM_IOCTL_FREE_BUFS =    DRM_IOCTL_FREE_BUFS;

const unsigned long FFI_DRM_IOCTL_RM_MAP =       DRM_IOCTL_RM_MAP;

const unsigned long FFI_DRM_IOCTL_SET_SAREA_CTX =    DRM_IOCTL_SET_SAREA_CTX;
const unsigned long FFI_DRM_IOCTL_GET_SAREA_CTX =    DRM_IOCTL_GET_SAREA_CTX;

const unsigned long FFI_DRM_IOCTL_SET_MASTER =   DRM_IOCTL_SET_MASTER;
const unsigned long FFI_DRM_IOCTL_DROP_MASTER =  DRM_IOCTL_DROP_MASTER;

const unsigned long FFI_DRM_IOCTL_ADD_CTX =      DRM_IOCTL_ADD_CTX;
const unsigned long FFI_DRM_IOCTL_RM_CTX =       DRM_IOCTL_RM_CTX;
const unsigned long FFI_DRM_IOCTL_MOD_CTX =      DRM_IOCTL_MOD_CTX;
const unsigned long FFI_DRM_IOCTL_GET_CTX =      DRM_IOCTL_GET_CTX;
const unsigned long FFI_DRM_IOCTL_SWITCH_CTX =   DRM_IOCTL_SWITCH_CTX;
const unsigned long FFI_DRM_IOCTL_NEW_CTX =      DRM_IOCTL_NEW_CTX;
const unsigned long FFI_DRM_IOCTL_RES_CTX =      DRM_IOCTL_RES_CTX;
const unsigned long FFI_DRM_IOCTL_ADD_DRAW =     DRM_IOCTL_ADD_DRAW;
const unsigned long FFI_DRM_IOCTL_RM_DRAW =      DRM_IOCTL_RM_DRAW;
const unsigned long FFI_DRM_IOCTL_DMA =          DRM_IOCTL_DMA;
const unsigned long FFI_DRM_IOCTL_LOCK =         DRM_IOCTL_LOCK;
const unsigned long FFI_DRM_IOCTL_UNLOCK =       DRM_IOCTL_UNLOCK;
const unsigned long FFI_DRM_IOCTL_FINISH =       DRM_IOCTL_FINISH;

const unsigned long FFI_DRM_IOCTL_PRIME_HANDLE_TO_FD =   DRM_IOCTL_PRIME_HANDLE_TO_FD;
const unsigned long FFI_DRM_IOCTL_PRIME_FD_TO_HANDLE =   DRM_IOCTL_PRIME_FD_TO_HANDLE;

const unsigned long FFI_DRM_IOCTL_AGP_ACQUIRE =  DRM_IOCTL_AGP_ACQUIRE;
const unsigned long FFI_DRM_IOCTL_AGP_RELEASE =  DRM_IOCTL_AGP_RELEASE;
const unsigned long FFI_DRM_IOCTL_AGP_ENABLE =   DRM_IOCTL_AGP_ENABLE;
const unsigned long FFI_DRM_IOCTL_AGP_INFO =     DRM_IOCTL_AGP_INFO;
const unsigned long FFI_DRM_IOCTL_AGP_ALLOC =    DRM_IOCTL_AGP_ALLOC;
const unsigned long FFI_DRM_IOCTL_AGP_FREE =     DRM_IOCTL_AGP_FREE;
const unsigned long FFI_DRM_IOCTL_AGP_BIND =     DRM_IOCTL_AGP_BIND;
const unsigned long FFI_DRM_IOCTL_AGP_UNBIND =   DRM_IOCTL_AGP_UNBIND;

const unsigned long FFI_DRM_IOCTL_SG_ALLOC = DRM_IOCTL_SG_ALLOC;
const unsigned long FFI_DRM_IOCTL_SG_FREE =  DRM_IOCTL_SG_FREE;

const unsigned long FFI_DRM_IOCTL_WAIT_VBLANK = DRM_IOCTL_WAIT_VBLANK;

const unsigned long FFI_DRM_IOCTL_UPDATE_DRAW = DRM_IOCTL_UPDATE_DRAW;

const unsigned long FFI_DRM_IOCTL_MODE_GETRESOURCES =    DRM_IOCTL_MODE_GETRESOURCES;
const unsigned long FFI_DRM_IOCTL_MODE_GETCRTC =         DRM_IOCTL_MODE_GETCRTC;
const unsigned long FFI_DRM_IOCTL_MODE_SETCRTC =         DRM_IOCTL_MODE_SETCRTC;
const unsigned long FFI_DRM_IOCTL_MODE_CURSOR =          DRM_IOCTL_MODE_CURSOR;
const unsigned long FFI_DRM_IOCTL_MODE_GETGAMMA =        DRM_IOCTL_MODE_GETGAMMA;
const unsigned long FFI_DRM_IOCTL_MODE_SETGAMMA =        DRM_IOCTL_MODE_SETGAMMA;
const unsigned long FFI_DRM_IOCTL_MODE_GETENCODER =      DRM_IOCTL_MODE_GETENCODER;
const unsigned long FFI_DRM_IOCTL_MODE_GETCONNECTOR =    DRM_IOCTL_MODE_GETCONNECTOR;
//const unsigned long FFI_DRM_IOCTL_MODE_ATTACHMODE =      DRM_IOWR(0xA8, struct drm_mode_mode_cmd); /* deprecated (never worked) */
//const unsigned long FFI_DRM_IOCTL_MODE_DETACHMODE =      DRM_IOWR(0xA9, struct drm_mode_mode_cmd); /* deprecated (never worked) */

const unsigned long FFI_DRM_IOCTL_MODE_GETPROPERTY = DRM_IOCTL_MODE_GETPROPERTY;
const unsigned long FFI_DRM_IOCTL_MODE_SETPROPERTY = DRM_IOCTL_MODE_SETPROPERTY;
const unsigned long FFI_DRM_IOCTL_MODE_GETPROPBLOB = DRM_IOCTL_MODE_GETPROPBLOB;
const unsigned long FFI_DRM_IOCTL_MODE_GETFB =       DRM_IOCTL_MODE_GETFB;
const unsigned long FFI_DRM_IOCTL_MODE_ADDFB =       DRM_IOCTL_MODE_ADDFB;
const unsigned long FFI_DRM_IOCTL_MODE_RMFB =        DRM_IOCTL_MODE_RMFB;
const unsigned long FFI_DRM_IOCTL_MODE_PAGE_FLIP =   DRM_IOCTL_MODE_PAGE_FLIP;
const unsigned long FFI_DRM_IOCTL_MODE_DIRTYFB =     DRM_IOCTL_MODE_DIRTYFB;

const unsigned long FFI_DRM_IOCTL_MODE_CREATE_DUMB =         DRM_IOCTL_MODE_CREATE_DUMB;
const unsigned long FFI_DRM_IOCTL_MODE_MAP_DUMB =            DRM_IOCTL_MODE_MAP_DUMB;
const unsigned long FFI_DRM_IOCTL_MODE_DESTROY_DUMB =        DRM_IOCTL_MODE_DESTROY_DUMB;
const unsigned long FFI_DRM_IOCTL_MODE_GETPLANERESOURCES =   DRM_IOCTL_MODE_GETPLANERESOURCES;
const unsigned long FFI_DRM_IOCTL_MODE_GETPLANE =            DRM_IOCTL_MODE_GETPLANE;
const unsigned long FFI_DRM_IOCTL_MODE_SETPLANE =            DRM_IOCTL_MODE_SETPLANE;
const unsigned long FFI_DRM_IOCTL_MODE_ADDFB2 =              DRM_IOCTL_MODE_ADDFB2;
const unsigned long FFI_DRM_IOCTL_MODE_OBJ_GETPROPERTIES =   DRM_IOCTL_MODE_OBJ_GETPROPERTIES;
const unsigned long FFI_DRM_IOCTL_MODE_OBJ_SETPROPERTY =     DRM_IOCTL_MODE_OBJ_SETPROPERTY;
const unsigned long FFI_DRM_IOCTL_MODE_CURSOR2 =             DRM_IOCTL_MODE_CURSOR2;
const unsigned long FFI_DRM_IOCTL_MODE_ATOMIC =              DRM_IOCTL_MODE_ATOMIC;
const unsigned long FFI_DRM_IOCTL_MODE_CREATEPROPBLOB =      DRM_IOCTL_MODE_CREATEPROPBLOB;
const unsigned long FFI_DRM_IOCTL_MODE_DESTROYPROPBLOB =     DRM_IOCTL_MODE_DESTROYPROPBLOB;

