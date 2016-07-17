#include <libdrm/drm_fourcc.h>

/* color index */
const unsigned int FFI_DRM_FORMAT_C8 = DRM_FORMAT_C8;

/* 8 bpp Red */
const unsigned int FFI_DRM_FORMAT_R8 = DRM_FORMAT_R8;

/* 16 bpp RG */
const unsigned int FFI_DRM_FORMAT_RG88 = DRM_FORMAT_RG88;
const unsigned int FFI_DRM_FORMAT_GR88 = DRM_FORMAT_GR88;

/* 8 bpp RGB */
const unsigned int FFI_DRM_FORMAT_RGB332 = DRM_FORMAT_RGB332;
const unsigned int FFI_DRM_FORMAT_BGR233 = DRM_FORMAT_BGR233;

/* 16 bpp RGB */
const unsigned int FFI_DRM_FORMAT_XRGB4444 = DRM_FORMAT_XRGB4444;
const unsigned int FFI_DRM_FORMAT_XBGR4444 = DRM_FORMAT_XBGR4444;
const unsigned int FFI_DRM_FORMAT_RGBX4444 = DRM_FORMAT_RGBX4444;
const unsigned int FFI_DRM_FORMAT_BGRX4444 = DRM_FORMAT_BGRX4444;

const unsigned int FFI_DRM_FORMAT_ARGB4444 = DRM_FORMAT_ARGB4444;
const unsigned int FFI_DRM_FORMAT_ABGR4444 = DRM_FORMAT_ABGR4444;
const unsigned int FFI_DRM_FORMAT_RGBA4444 = DRM_FORMAT_RGBA4444;
const unsigned int FFI_DRM_FORMAT_BGRA4444 = DRM_FORMAT_BGRA4444;

const unsigned int FFI_DRM_FORMAT_XRGB1555 = DRM_FORMAT_XRGB1555;
const unsigned int FFI_DRM_FORMAT_XBGR1555 = DRM_FORMAT_XBGR1555;
const unsigned int FFI_DRM_FORMAT_RGBX5551 = DRM_FORMAT_RGBX5551;
const unsigned int FFI_DRM_FORMAT_BGRX5551 = DRM_FORMAT_BGRX5551;

const unsigned int FFI_DRM_FORMAT_ARGB1555 = DRM_FORMAT_ARGB1555;
const unsigned int FFI_DRM_FORMAT_ABGR1555 = DRM_FORMAT_ABGR1555;
const unsigned int FFI_DRM_FORMAT_RGBA5551 = DRM_FORMAT_RGBA5551;
const unsigned int FFI_DRM_FORMAT_BGRA5551 = DRM_FORMAT_BGRA5551;

const unsigned int FFI_DRM_FORMAT_RGB565 = DRM_FORMAT_RGB565;
const unsigned int FFI_DRM_FORMAT_BGR565 = DRM_FORMAT_BGR565;

/* 24 bpp RGB */
const unsigned int FFI_DRM_FORMAT_RGB888 = DRM_FORMAT_RGB888;
const unsigned int FFI_DRM_FORMAT_BGR888 = DRM_FORMAT_BGR888;

/* 32 bpp RGB */
const unsigned int FFI_DRM_FORMAT_XRGB8888 = DRM_FORMAT_XRGB8888;
const unsigned int FFI_DRM_FORMAT_XBGR8888 = DRM_FORMAT_XBGR8888;
const unsigned int FFI_DRM_FORMAT_RGBX8888 = DRM_FORMAT_RGBX8888;
const unsigned int FFI_DRM_FORMAT_BGRX8888 = DRM_FORMAT_BGRX8888;

const unsigned int FFI_DRM_FORMAT_ARGB8888 = DRM_FORMAT_ARGB8888;
const unsigned int FFI_DRM_FORMAT_ABGR8888 = DRM_FORMAT_ABGR8888;
const unsigned int FFI_DRM_FORMAT_RGBA8888 = DRM_FORMAT_RGBA8888;
const unsigned int FFI_DRM_FORMAT_BGRA8888 = DRM_FORMAT_BGRA8888;

const unsigned int FFI_DRM_FORMAT_XRGB2101010 = DRM_FORMAT_XRGB2101010;
const unsigned int FFI_DRM_FORMAT_XBGR2101010 = DRM_FORMAT_XBGR2101010;
const unsigned int FFI_DRM_FORMAT_RGBX1010102 = DRM_FORMAT_RGBX1010102;
const unsigned int FFI_DRM_FORMAT_BGRX1010102 = DRM_FORMAT_BGRX1010102;

const unsigned int FFI_DRM_FORMAT_ARGB2101010 = DRM_FORMAT_ARGB2101010;
const unsigned int FFI_DRM_FORMAT_ABGR2101010 = DRM_FORMAT_ABGR2101010;
const unsigned int FFI_DRM_FORMAT_RGBA1010102 = DRM_FORMAT_RGBA1010102;
const unsigned int FFI_DRM_FORMAT_BGRA1010102 = DRM_FORMAT_BGRA1010102;

/* packed YCbCr */
const unsigned int FFI_DRM_FORMAT_YUYV = DRM_FORMAT_YUYV;
const unsigned int FFI_DRM_FORMAT_YVYU = DRM_FORMAT_YVYU;
const unsigned int FFI_DRM_FORMAT_UYVY = DRM_FORMAT_UYVY;
const unsigned int FFI_DRM_FORMAT_VYUY = DRM_FORMAT_VYUY;

const unsigned int FFI_DRM_FORMAT_AYUV = DRM_FORMAT_AYUV;

/*
 * 2 plane YCbCr
 * index 0 = Y plane, [7:0] Y
 * index 1 = Cr:Cb plane, [15:0] Cr:Cb little endian
 * or
 * index 1 = Cb:Cr plane, [15:0] Cb:Cr little endian
 */
const unsigned int FFI_DRM_FORMAT_NV12 = DRM_FORMAT_NV12;
const unsigned int FFI_DRM_FORMAT_NV21 = DRM_FORMAT_NV21;
const unsigned int FFI_DRM_FORMAT_NV16 = DRM_FORMAT_NV16;
const unsigned int FFI_DRM_FORMAT_NV61 = DRM_FORMAT_NV61;
const unsigned int FFI_DRM_FORMAT_NV24 = DRM_FORMAT_NV24;
const unsigned int FFI_DRM_FORMAT_NV42 = DRM_FORMAT_NV42;

/*
 * 3 plane YCbCr
 * index 0: Y plane, [7:0] Y
 * index 1: Cb plane, [7:0] Cb
 * index 2: Cr plane, [7:0] Cr
 * or
 * index 1: Cr plane, [7:0] Cr
 * index 2: Cb plane, [7:0] Cb
 */
const unsigned int FFI_DRM_FORMAT_YUV410 = DRM_FORMAT_YUV410;
const unsigned int FFI_DRM_FORMAT_YVU410 = DRM_FORMAT_YVU410;
const unsigned int FFI_DRM_FORMAT_YUV411 = DRM_FORMAT_YUV411;
const unsigned int FFI_DRM_FORMAT_YVU411 = DRM_FORMAT_YVU411;
const unsigned int FFI_DRM_FORMAT_YUV420 = DRM_FORMAT_YUV420;
const unsigned int FFI_DRM_FORMAT_YVU420 = DRM_FORMAT_YVU420;
const unsigned int FFI_DRM_FORMAT_YUV422 = DRM_FORMAT_YUV422;
const unsigned int FFI_DRM_FORMAT_YVU422 = DRM_FORMAT_YVU422;
const unsigned int FFI_DRM_FORMAT_YUV444 = DRM_FORMAT_YUV444;
const unsigned int FFI_DRM_FORMAT_YVU444 = DRM_FORMAT_YVU444;

