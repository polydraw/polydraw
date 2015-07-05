#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

pub use libc::{c_char, c_uchar, c_short, c_ushort, c_int, c_uint, c_long,
   c_ulong, c_void, c_float, free};
use std::mem;

pub enum XDisplay { }
pub enum XPrivate { }
pub enum XrmHashBucketRec { }
pub enum XGC { }

pub enum xcb_connection_t { }

pub type uint8_t = c_uchar;
pub type int16_t = c_short;
pub type uint16_t = c_ushort;
pub type int32_t = c_int;
pub type uint32_t = c_uint;

pub type XID = c_ulong;
pub type GC = *mut XGC;
pub type Display = XDisplay;
pub type Colormap = XID;
pub type Window = XID;
pub type XPointer = *mut c_char;
pub type VisualID = c_ulong;

pub type xcb_keycode_t = uint8_t;
pub type xcb_window_t = uint32_t;
pub type xcb_colormap_t = uint32_t;
pub type xcb_visualid_t = uint32_t;

pub type XEventQueueOwner = c_uint;

pub const XlibOwnsEventQueue: c_uint = 0;
pub const XCBOwnsEventQueue: c_uint = 1;

pub const XCB_COLORMAP_ALLOC_NONE: c_uint = 0;
pub const XCB_COLORMAP_ALLOC_ALL: c_uint = 1;

pub const XCB_EVENT_MASK_NO_EVENT:               c_uint = 0;
pub const XCB_EVENT_MASK_KEY_PRESS:              c_uint = 1;
pub const XCB_EVENT_MASK_KEY_RELEASE:            c_uint = 2;
pub const XCB_EVENT_MASK_BUTTON_PRESS:           c_uint = 4;
pub const XCB_EVENT_MASK_BUTTON_RELEASE:         c_uint = 8;
pub const XCB_EVENT_MASK_ENTER_WINDOW:           c_uint = 16;
pub const XCB_EVENT_MASK_LEAVE_WINDOW:           c_uint = 32;
pub const XCB_EVENT_MASK_POINTER_MOTION:         c_uint = 64;
pub const XCB_EVENT_MASK_POINTER_MOTION_HINT:    c_uint = 128;
pub const XCB_EVENT_MASK_BUTTON_1_MOTION:        c_uint = 256;
pub const XCB_EVENT_MASK_BUTTON_2_MOTION:        c_uint = 512;
pub const XCB_EVENT_MASK_BUTTON_3_MOTION:        c_uint = 1024;
pub const XCB_EVENT_MASK_BUTTON_4_MOTION:        c_uint = 2048;
pub const XCB_EVENT_MASK_BUTTON_5_MOTION:        c_uint = 4096;
pub const XCB_EVENT_MASK_BUTTON_MOTION:          c_uint = 8192;
pub const XCB_EVENT_MASK_KEYMAP_STATE:           c_uint = 16384;
pub const XCB_EVENT_MASK_EXPOSURE:               c_uint = 32768;
pub const XCB_EVENT_MASK_VISIBILITY_CHANGE:      c_uint = 65536;
pub const XCB_EVENT_MASK_STRUCTURE_NOTIFY:       c_uint = 131072;
pub const XCB_EVENT_MASK_RESIZE_REDIRECT:        c_uint = 262144;
pub const XCB_EVENT_MASK_SUBSTRUCTURE_NOTIFY:    c_uint = 524288;
pub const XCB_EVENT_MASK_SUBSTRUCTURE_REDIRECT:  c_uint = 1048576;
pub const XCB_EVENT_MASK_FOCUS_CHANGE:           c_uint = 2097152;
pub const XCB_EVENT_MASK_PROPERTY_CHANGE:        c_uint = 4194304;
pub const XCB_EVENT_MASK_COLOR_MAP_CHANGE:       c_uint = 8388608;
pub const XCB_EVENT_MASK_OWNER_GRAB_BUTTON:      c_uint = 16777216;

pub const XCB_CW_BACK_PIXMAP:        c_uint = 1;
pub const XCB_CW_BACK_PIXEL:         c_uint = 2;
pub const XCB_CW_BORDER_PIXMAP:      c_uint = 4;
pub const XCB_CW_BORDER_PIXEL:       c_uint = 8;
pub const XCB_CW_BIT_GRAVITY:        c_uint = 16;
pub const XCB_CW_WIN_GRAVITY:        c_uint = 32;
pub const XCB_CW_BACKING_STORE:      c_uint = 64;
pub const XCB_CW_BACKING_PLANES:     c_uint = 128;
pub const XCB_CW_BACKING_PIXEL:      c_uint = 256;
pub const XCB_CW_OVERRIDE_REDIRECT:  c_uint = 512;
pub const XCB_CW_SAVE_UNDER:         c_uint = 1024;
pub const XCB_CW_EVENT_MASK:         c_uint = 2048;
pub const XCB_CW_DONT_PROPAGATE:     c_uint = 4096;
pub const XCB_CW_COLORMAP:           c_uint = 8192;
pub const XCB_CW_CURSOR:             c_uint = 16384;

pub const XCB_COPY_FROM_PARENT:  c_uint = 0;

pub const XCB_WINDOW_CLASS_COPY_FROM_PARENT:  c_uint = 0;
pub const XCB_WINDOW_CLASS_INPUT_OUTPUT:      c_uint = 1;
pub const XCB_WINDOW_CLASS_INPUT_ONLY:        c_uint = 2;

pub const XCB_KEY_PRESS:  c_uchar = 2;
pub const XCB_EXPOSE:     c_uchar = 12;

#[repr(C)]
#[derive(Copy)]
pub struct XExtData {
   pub number: c_int,
   pub next: *mut XExtData,
   pub free_private: Option<extern "C" fn(extension: *mut XExtData) -> c_int>,
   pub private_data: XPointer,
}
impl Clone for XExtData {
   fn clone(&self) -> Self { *self }
}
impl Default for XExtData {
   fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct Visual {
   pub ext_data: *mut XExtData,
   pub visualid: VisualID,
   pub class: c_int,
   pub red_mask: c_ulong,
   pub green_mask: c_ulong,
   pub blue_mask: c_ulong,
   pub bits_per_rgb: c_int,
   pub map_entries: c_int,
}
impl Clone for Visual {
   fn clone(&self) -> Self { *self }
}
impl Default for Visual {
   fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct Depth {
   pub depth: c_int,
   pub nvisuals: c_int,
   pub visuals: *mut Visual,
}
impl Clone for Depth {
   fn clone(&self) -> Self { *self }
}
impl Default for Depth {
   fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct Screen {
   pub ext_data: *mut XExtData,
   pub display: *mut XDisplay,
   pub root: Window,
   pub width: c_int,
   pub height: c_int,
   pub mwidth: c_int,
   pub mheight: c_int,
   pub ndepths: c_int,
   pub depths: *mut Depth,
   pub root_depth: c_int,
   pub root_visual: *mut Visual,
   pub default_gc: GC,
   pub cmap: Colormap,
   pub white_pixel: c_ulong,
   pub black_pixel: c_ulong,
   pub max_maps: c_int,
   pub min_maps: c_int,
   pub backing_store: c_int,
   pub save_unders: c_int,
   pub root_input_mask: c_long,
}
impl Clone for Screen {
   fn clone(&self) -> Self { *self }
}
impl Default for Screen {
   fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct ScreenFormat {
   pub ext_data: *mut XExtData,
   pub depth: c_int,
   pub bits_per_pixel: c_int,
   pub scanline_pad: c_int,
}
impl Clone for ScreenFormat {
   fn clone(&self) -> Self { *self }
}
impl Default for ScreenFormat {
   fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct XPrivDisplay {
   pub ext_data: *mut XExtData,
   pub private1: *mut XPrivate,
   pub fd: c_int,
   pub private2: c_int,
   pub proto_major_version: c_int,
   pub proto_minor_version: c_int,
   pub vendor: *mut c_char,
   pub private3: XID,
   pub private4: XID,
   pub private5: XID,
   pub private6: c_int,
   pub resource_alloc: Option<extern "C" fn(arg1: *mut XDisplay) -> XID>,
   pub byte_order: c_int,
   pub bitmap_unit: c_int,
   pub bitmap_pad: c_int,
   pub bitmap_bit_order: c_int,
   pub nformats: c_int,
   pub pixmap_format: *mut ScreenFormat,
   pub private8: c_int,
   pub release: c_int,
   pub private9: *mut XPrivate,
   pub private10: *mut XPrivate,
   pub qlen: c_int,
   pub last_request_read: c_ulong,
   pub request: c_ulong,
   pub private11: XPointer,
   pub private12: XPointer,
   pub private13: XPointer,
   pub private14: XPointer,
   pub max_request_size: c_uint,
   pub db: *mut XrmHashBucketRec,
   pub private15: Option<extern "C" fn(arg1: *mut XDisplay) -> c_int>,
   pub display_name: *mut c_char,
   pub default_screen: c_int,
   pub nscreens: c_int,
   pub screens: *mut Screen,
   pub motion_buffer: c_ulong,
   pub private16: c_ulong,
   pub min_keycode: c_int,
   pub max_keycode: c_int,
   pub private17: XPointer,
   pub private18: XPointer,
   pub private19: c_int,
   pub xdefaults: *mut c_char,
}
impl Clone for XPrivDisplay {
   fn clone(&self) -> Self { *self }
}
impl Default for XPrivDisplay {
   fn default() -> Self { unsafe { mem::zeroed() } }
}
pub type _XPrivDisplay = *mut XPrivDisplay;

#[repr(C)]
#[derive(Copy)]
pub struct xcb_screen_t {
   pub root: xcb_window_t,
   pub default_colormap: xcb_colormap_t,
   pub white_pixel: uint32_t,
   pub black_pixel: uint32_t,
   pub current_input_masks: uint32_t,
   pub width_in_pixels: uint16_t,
   pub height_in_pixels: uint16_t,
   pub width_in_millimeters: uint16_t,
   pub height_in_millimeters: uint16_t,
   pub min_installed_maps: uint16_t,
   pub max_installed_maps: uint16_t,
   pub root_visual: xcb_visualid_t,
   pub backing_stores: uint8_t,
   pub save_unders: uint8_t,
   pub root_depth: uint8_t,
   pub allowed_depths_len: uint8_t,
}
impl ::std::clone::Clone for xcb_screen_t {
   fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for xcb_screen_t {
   fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct xcb_screen_iterator_t {
   pub data: *mut xcb_screen_t,
   pub rem: c_int,
   pub index: c_int,
}
impl ::std::clone::Clone for xcb_screen_iterator_t {
   fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for xcb_screen_iterator_t {
   fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct xcb_setup_t {
   pub status: uint8_t,
   pub pad0: uint8_t,
   pub protocol_major_version: uint16_t,
   pub protocol_minor_version: uint16_t,
   pub length: uint16_t,
   pub release_number: uint32_t,
   pub resource_id_base: uint32_t,
   pub resource_id_mask: uint32_t,
   pub motion_buffer_size: uint32_t,
   pub vendor_len: uint16_t,
   pub maximum_request_length: uint16_t,
   pub roots_len: uint8_t,
   pub pixmap_formats_len: uint8_t,
   pub image_byte_order: uint8_t,
   pub bitmap_format_bit_order: uint8_t,
   pub bitmap_format_scanline_unit: uint8_t,
   pub bitmap_format_scanline_pad: uint8_t,
   pub min_keycode: xcb_keycode_t,
   pub max_keycode: xcb_keycode_t,
   pub pad1: [uint8_t; 4usize],
}
impl ::std::clone::Clone for xcb_setup_t {
   fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for xcb_setup_t {
   fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct xcb_void_cookie_t {
   pub sequence: c_uint,
}
impl ::std::clone::Clone for xcb_void_cookie_t {
   fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for xcb_void_cookie_t {
   fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct xcb_generic_event_t {
   pub response_type: uint8_t,
   pub pad0: uint8_t,
   pub sequence: uint16_t,
   pub pad: [uint32_t; 7usize],
   pub full_sequence: uint32_t,
}
impl ::std::clone::Clone for xcb_generic_event_t {
   fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for xcb_generic_event_t {
   fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct xcb_generic_error_t {
   pub response_type: uint8_t,
   pub error_code: uint8_t,
   pub sequence: uint16_t,
   pub resource_id: uint32_t,
   pub minor_code: uint16_t,
   pub major_code: uint8_t,
   pub pad0: uint8_t,
   pub pad: [uint32_t; 5usize],
   pub full_sequence: uint32_t,
}
impl ::std::clone::Clone for xcb_generic_error_t {
   fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for xcb_generic_error_t {
   fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

#[link(name="X11")]
extern "C" {
   pub fn XOpenDisplay(display_name: *const c_char) -> *mut Display;
   pub fn XCloseDisplay(display: *mut Display) -> c_int;
   pub fn XFree(data: *mut c_void) -> c_int;
}

#[link(name="X11-xcb")]
extern "C" {
   pub fn XGetXCBConnection(display: *mut Display) -> *mut xcb_connection_t;
   pub fn XSetEventQueueOwner(
      display: *mut Display,
      owner: XEventQueueOwner
   ) -> ();
}

#[link(name="xcb")]
extern "C" {
   pub fn xcb_get_setup(c: *mut xcb_connection_t) -> *const xcb_setup_t;

   pub fn xcb_generate_id(c: *mut xcb_connection_t) -> uint32_t;

   pub fn xcb_flush(c: *mut xcb_connection_t) -> c_int;

   pub fn xcb_screen_next(i: *mut xcb_screen_iterator_t) -> ();

   pub fn xcb_connect(
      displayname: *const c_char,
      screenp: *mut c_int
   ) -> *mut xcb_connection_t;

   pub fn xcb_connection_has_error(c: *mut xcb_connection_t) -> c_int;

   pub fn xcb_setup_roots_iterator(
      R: *const xcb_setup_t
   ) -> xcb_screen_iterator_t;

   pub fn xcb_create_colormap(
      c: *mut xcb_connection_t,
      alloc: uint8_t,
      mid: xcb_colormap_t,
      window: xcb_window_t,
      visual: xcb_visualid_t
   ) -> xcb_void_cookie_t;

   pub fn xcb_create_window(
      c: *mut xcb_connection_t,
      depth: uint8_t,
      wid: xcb_window_t,
      parent: xcb_window_t,
      x: int16_t,
      y: int16_t,
      width: uint16_t,
      height: uint16_t,
      border_width: uint16_t,
      _class: uint16_t,
      visual: xcb_visualid_t,
      value_mask: uint32_t,
      value_list: *const uint32_t
   ) -> xcb_void_cookie_t;

   pub fn xcb_destroy_window(
      c: *mut xcb_connection_t,
      window: xcb_window_t
   ) -> xcb_void_cookie_t;

   pub fn xcb_map_window(
      c: *mut xcb_connection_t,
      window: xcb_window_t
   ) -> xcb_void_cookie_t;

   pub fn xcb_wait_for_event(
      c: *mut xcb_connection_t
   ) -> *mut xcb_generic_event_t;
}

pub type GLenum = c_uint;
pub type GLbitfield = c_uint;
pub type GLclampf = c_float;

pub const GL_CURRENT_BIT:          GLenum = 0x00000001;
pub const GL_POINT_BIT:            GLenum = 0x00000002;
pub const GL_LINE_BIT:             GLenum = 0x00000004;
pub const GL_POLYGON_BIT:          GLenum = 0x00000008;
pub const GL_POLYGON_STIPPLE_BIT:  GLenum = 0x00000010;
pub const GL_PIXEL_MODE_BIT:       GLenum = 0x00000020;
pub const GL_LIGHTING_BIT:         GLenum = 0x00000040;
pub const GL_FOG_BIT:              GLenum = 0x00000080;
pub const GL_DEPTH_BUFFER_BIT:     GLenum = 0x00000100;
pub const GL_ACCUM_BUFFER_BIT:     GLenum = 0x00000200;
pub const GL_STENCIL_BUFFER_BIT:   GLenum = 0x00000400;
pub const GL_VIEWPORT_BIT:         GLenum = 0x00000800;
pub const GL_TRANSFORM_BIT:        GLenum = 0x00001000;
pub const GL_ENABLE_BIT:           GLenum = 0x00002000;
pub const GL_COLOR_BUFFER_BIT:     GLenum = 0x00004000;
pub const GL_HINT_BIT:             GLenum = 0x00008000;
pub const GL_EVAL_BIT:             GLenum = 0x00010000;
pub const GL_LIST_BIT:             GLenum = 0x00020000;
pub const GL_TEXTURE_BIT:          GLenum = 0x00040000;
pub const GL_SCISSOR_BIT:          GLenum = 0x00080000;
pub const GL_ALL_ATTRIB_BITS:      GLenum = 0xFFFFFFFF;

#[link(name="GL")]
extern "C" {
   pub fn glClearColor(
      red: GLclampf,
      green: GLclampf,
      blue: GLclampf,
      alpha:
      GLclampf
   ) -> ();

   pub fn glClear(
      mask: GLbitfield
   ) -> ();

   pub fn glFlush() -> ();
}

pub type khronos_int32_t = int32_t;

pub type EGLenum = c_uint;
pub type EGLint = khronos_int32_t;
pub type EGLBoolean = c_uint;
pub type EGLDisplay = *mut c_void;
pub type EGLConfig = *mut c_void;
pub type EGLContext = *mut c_void;
pub type EGLSurface = *mut c_void;

pub type EGLNativeDisplayType = *mut Display;
pub type EGLNativeWindowType = xcb_window_t;

pub const EGL_NO_CONTEXT:                   EGLContext = 0 as EGLContext;

pub const EGL_BUFFER_SIZE:                  EGLenum = 0x3020;
pub const EGL_ALPHA_SIZE:                   EGLenum = 0x3021;
pub const EGL_BLUE_SIZE:                    EGLenum = 0x3022;
pub const EGL_GREEN_SIZE:                   EGLenum = 0x3023;
pub const EGL_RED_SIZE:                     EGLenum = 0x3024;
pub const EGL_DEPTH_SIZE:                   EGLenum = 0x3025;
pub const EGL_STENCIL_SIZE:                 EGLenum = 0x3026;
pub const EGL_CONFIG_CAVEAT:                EGLenum = 0x3027;
pub const EGL_CONFIG_ID:                    EGLenum = 0x3028;
pub const EGL_LEVEL:                        EGLenum = 0x3029;
pub const EGL_MAX_PBUFFER_HEIGHT:           EGLenum = 0x302A;
pub const EGL_MAX_PBUFFER_PIXELS:           EGLenum = 0x302B;
pub const EGL_MAX_PBUFFER_WIDTH:            EGLenum = 0x302C;
pub const EGL_NATIVE_RENDERABLE:            EGLenum = 0x302D;
pub const EGL_NATIVE_VISUAL_ID:             EGLenum = 0x302E;
pub const EGL_NATIVE_VISUAL_TYPE:           EGLenum = 0x302F;
pub const EGL_SAMPLES:                      EGLenum = 0x3031;
pub const EGL_SAMPLE_BUFFERS:               EGLenum = 0x3032;
pub const EGL_SURFACE_TYPE:                 EGLenum = 0x3033;
pub const EGL_TRANSPARENT_TYPE:             EGLenum = 0x3034;
pub const EGL_TRANSPARENT_BLUE_VALUE:       EGLenum = 0x3035;
pub const EGL_TRANSPARENT_GREEN_VALUE:      EGLenum = 0x3036;
pub const EGL_TRANSPARENT_RED_VALUE:        EGLenum = 0x3037;
pub const EGL_NONE:                         EGLenum = 0x3038;
pub const EGL_BIND_TO_TEXTURE_RGB:          EGLenum = 0x3039;
pub const EGL_BIND_TO_TEXTURE_RGBA:         EGLenum = 0x303A;
pub const EGL_MIN_SWAP_INTERVAL:            EGLenum = 0x303B;
pub const EGL_MAX_SWAP_INTERVAL:            EGLenum = 0x303C;
pub const EGL_LUMINANCE_SIZE:               EGLenum = 0x303D;
pub const EGL_ALPHA_MASK_SIZE:              EGLenum = 0x303E;
pub const EGL_COLOR_BUFFER_TYPE:            EGLenum = 0x303F;
pub const EGL_RENDERABLE_TYPE:              EGLenum = 0x3040;
pub const EGL_MATCH_NATIVE_PIXMAP:          EGLenum = 0x3041;
pub const EGL_CONFORMANT:                   EGLenum = 0x3042;

pub const EGL_SLOW_CONFIG:                  EGLenum = 0x3050;
pub const EGL_NON_CONFORMANT_CONFIG:        EGLenum = 0x3051;
pub const EGL_TRANSPARENT_RGB:              EGLenum = 0x3052;
pub const EGL_RGB_BUFFER:                   EGLenum = 0x308E;
pub const EGL_LUMINANCE_BUFFER:             EGLenum = 0x308F;

pub const EGL_HEIGHT:                       EGLenum = 0x3056;
pub const EGL_WIDTH:                        EGLenum = 0x3057;
pub const EGL_LARGEST_PBUFFER:              EGLenum = 0x3058;
pub const EGL_TEXTURE_FORMAT:               EGLenum = 0x3080;
pub const EGL_TEXTURE_TARGET:               EGLenum = 0x3081;
pub const EGL_MIPMAP_TEXTURE:               EGLenum = 0x3082;
pub const EGL_MIPMAP_LEVEL:                 EGLenum = 0x3083;
pub const EGL_RENDER_BUFFER:                EGLenum = 0x3086;
pub const EGL_VG_COLORSPACE:                EGLenum = 0x3087;
pub const EGL_VG_ALPHA_FORMAT:              EGLenum = 0x3088;
pub const EGL_HORIZONTAL_RESOLUTION:        EGLenum = 0x3090;
pub const EGL_VERTICAL_RESOLUTION:          EGLenum = 0x3091;
pub const EGL_PIXEL_ASPECT_RATIO:           EGLenum = 0x3092;
pub const EGL_SWAP_BEHAVIOR:                EGLenum = 0x3093;
pub const EGL_MULTISAMPLE_RESOLVE:          EGLenum = 0x3099;

pub const EGL_BACK_BUFFER:                  EGLenum = 0x3084;
pub const EGL_SINGLE_BUFFER:                EGLenum = 0x3085;

pub const EGL_OPENGL_ES_API:                EGLenum = 0x30A0;
pub const EGL_OPENVG_API:                   EGLenum = 0x30A1;
pub const EGL_OPENGL_API:                   EGLenum = 0x30A2;

pub const EGL_PBUFFER_BIT:                  EGLenum = 0x0001;
pub const EGL_PIXMAP_BIT:                   EGLenum = 0x0002;
pub const EGL_WINDOW_BIT:                   EGLenum = 0x0004;
pub const EGL_VG_COLORSPACE_LINEAR_BIT:     EGLenum = 0x0020;
pub const EGL_VG_ALPHA_FORMAT_PRE_BIT:      EGLenum = 0x0040;
pub const EGL_MULTISAMPLE_RESOLVE_BOX_BIT:  EGLenum = 0x0200;
pub const EGL_SWAP_BEHAVIOR_PRESERVED_BIT:  EGLenum = 0x0400;

pub const EGL_OPENGL_ES_BIT:                EGLenum = 0x0001;
pub const EGL_OPENVG_BIT:                   EGLenum = 0x0002;
pub const EGL_OPENGL_ES2_BIT:               EGLenum = 0x0004;
pub const EGL_OPENGL_BIT:                   EGLenum = 0x0008;

pub type __eglMustCastToProperFunctionPointerType =
//   Option<extern "C" fn() -> ()>;
   *const c_void;

#[link(name="EGL")]
extern "C" {
   pub fn eglBindAPI(
      api: EGLenum
   ) -> EGLBoolean;

   pub fn eglGetDisplay(
      display_id: EGLNativeDisplayType
   ) -> EGLDisplay;

   pub fn eglInitialize(
      dpy: EGLDisplay,
      major: *mut EGLint,
      minor: *mut EGLint
   ) -> EGLBoolean;

   pub fn eglChooseConfig(
      dpy: EGLDisplay,
      attrib_list: *const EGLint,
      configs: *mut EGLConfig,
      config_size: EGLint,
      num_config: *mut EGLint
   ) -> EGLBoolean;

   pub fn eglGetProcAddress(
      procname: *const c_char
   ) -> __eglMustCastToProperFunctionPointerType;

   pub fn eglCreateContext(
      dpy: EGLDisplay,
      config: EGLConfig,
      share_context: EGLContext,
      attrib_list: *const EGLint
   ) -> EGLContext;

   pub fn eglCreateWindowSurface(
      dpy: EGLDisplay,
      config: EGLConfig,
      win: EGLNativeWindowType,
      attrib_list: *const EGLint
   ) -> EGLSurface;

   pub fn eglMakeCurrent(
      dpy: EGLDisplay,
      draw: EGLSurface,
      read: EGLSurface,
      ctx: EGLContext
   ) -> EGLBoolean;

   pub fn eglQueryContext(
      dpy: EGLDisplay,
      ctx: EGLContext,
      attribute: EGLint,
      value: *mut EGLint
   ) -> EGLBoolean;

   pub fn eglSwapBuffers(
      dpy: EGLDisplay,
      surface: EGLSurface
   ) -> EGLBoolean;
}

#[macro_export]
macro_rules! DefaultScreen {
   ($display:expr) => (
      unsafe{
         (
            *($display as $crate::platform::x11::ffi::_XPrivDisplay)
         ).default_screen
      }
   )
}
