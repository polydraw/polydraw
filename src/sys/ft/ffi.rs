#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use std::mem;

use std::os::raw::{
   c_char, c_short, c_int, c_long, c_void, c_uchar, c_ushort, c_uint, c_ulong,

};

use sys::utils::fn_ptr::{FnPtr, NULL_PTR, FnPtrLoader};


pub type FT_Bool = c_uchar;
pub type FT_FWord = c_short;
pub type FT_UFWord = c_ushort;
pub type FT_Char = c_char;
pub type FT_Byte = c_uchar;
pub type FT_Bytes = *const FT_Byte;
pub type FT_Tag = FT_UInt32;
pub type FT_String = c_char;
pub type FT_Short = c_short;
pub type FT_UShort = c_ushort;
pub type FT_Int = c_int;
pub type FT_UInt = c_uint;
pub type FT_Long = c_long;
pub type FT_ULong = c_ulong;
pub type FT_F2Dot14 = c_short;
pub type FT_F26Dot6 = c_long;
pub type FT_Fixed = c_long;
pub type FT_Error = c_int;
pub type FT_Pointer = *mut c_void;
pub type FT_Int16 = c_short;
pub type FT_UInt16 = c_ushort;
pub type FT_Int32 = c_int;
pub type FT_UInt32 = c_uint;
pub type FT_Fast = c_int;
pub type FT_UFast = c_uint;
pub type FT_Int64 = c_long;
pub type FT_UInt64 = c_ulong;
pub type FT_Pos = c_long;

pub type FT_Generic_Finalizer = Option<unsafe extern "C" fn(
   object: *mut c_void
)>;

pub type FT_Outline_MoveToFunc = Option<unsafe extern "C" fn(
   to: *const FT_Vector,
   user: *mut c_void
) -> c_int>;

pub type FT_Outline_LineToFunc = Option<unsafe extern "C" fn(
   to: *const FT_Vector,
   user: *mut c_void
) -> c_int>;

pub type FT_Outline_ConicToFunc = Option<unsafe extern "C" fn(
   control: *const FT_Vector,
   to: *const FT_Vector,
   user: *mut c_void
) -> c_int>;

pub type FT_Outline_CubicToFunc = Option<unsafe extern "C" fn(
   control1: *const FT_Vector,
   control2: *const FT_Vector,
   to: *const FT_Vector,
   user: *mut c_void
) -> c_int>;

pub type FT_Alloc_Func = Option<extern "C" fn(
   memory: FT_Memory,
   size: c_long
) -> *mut c_void>;

pub type FT_Free_Func = Option<unsafe extern "C" fn(
   memory: FT_Memory,
   block: *mut c_void
)>;

pub type FT_Realloc_Func = Option<unsafe extern "C" fn(
   memory: FT_Memory,
   cur_size: c_long,
   new_size: c_long,
   block: c_void
) -> *mut c_void>;

pub type FT_Stream_IoFunc = Option<unsafe extern "C" fn(
   stream: FT_Stream,
   offset: c_ulong,
   buffer: c_uchar,
   count: c_ulong
) -> c_ulong>;

pub type FT_Stream_CloseFunc = Option<extern "C" fn(
   stream: FT_Stream
)>;

pub const FT_LOAD_DEFAULT:                         FT_Int32 = 0x0;
pub const FT_LOAD_NO_SCALE:                        FT_Int32 = 0x1 << 0;
pub const FT_LOAD_NO_HINTING:                      FT_Int32 = 0x1 << 1;
pub const FT_LOAD_RENDER:                          FT_Int32 = 0x1 << 2;
pub const FT_LOAD_NO_BITMAP:                       FT_Int32 = 0x1 << 3;
pub const FT_LOAD_VERTICAL_LAYOUT:                 FT_Int32 = 0x1 << 4;
pub const FT_LOAD_FORCE_AUTOHINT:                  FT_Int32 = 0x1 << 5;
pub const FT_LOAD_CROP_BITMAP:                     FT_Int32 = 0x1 << 6;
pub const FT_LOAD_PEDANTIC:                        FT_Int32 = 0x1 << 7;
pub const FT_LOAD_IGNORE_GLOBAL_ADVANCE_WIDTH:     FT_Int32 = 0x1 << 9;
pub const FT_LOAD_NO_RECURSE:                      FT_Int32 = 0x1 << 10;
pub const FT_LOAD_IGNORE_TRANSFORM:                FT_Int32 = 0x1 << 11;
pub const FT_LOAD_MONOCHROME:                      FT_Int32 = 0x1 << 12;
pub const FT_LOAD_LINEAR_DESIGN:                   FT_Int32 = 0x1 << 13;
pub const FT_LOAD_NO_AUTOHINT:                     FT_Int32 = 0x1 << 15;
pub const FT_LOAD_COLOR:                           FT_Int32 = 0x1 << 20;

pub const FT_KERNING_DEFAULT:                      FT_UInt = 0;
pub const FT_KERNING_UNFITTED:                     FT_UInt = 1;
pub const FT_KERNING_UNSCALED:                     FT_UInt = 2;


#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum FT_Encoding {
   FT_ENCODING_NONE = 0,
   FT_ENCODING_MS_SYMBOL = 1937337698,
   FT_ENCODING_UNICODE = 1970170211,
   FT_ENCODING_SJIS = 1936353651,
   FT_ENCODING_GB2312 = 1734484000,
   FT_ENCODING_BIG5 = 1651074869,
   FT_ENCODING_WANSUNG = 2002873971,
   FT_ENCODING_JOHAB = 1785686113,
   FT_ENCODING_ADOBE_STANDARD = 1094995778,
   FT_ENCODING_ADOBE_EXPERT = 1094992453,
   FT_ENCODING_ADOBE_CUSTOM = 1094992451,
   FT_ENCODING_ADOBE_LATIN_1 = 1818326065,
   FT_ENCODING_OLD_LATIN_2 = 1818326066,
   FT_ENCODING_APPLE_ROMAN = 1634889070,
}

#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum FT_Glyph_Format {
   FT_GLYPH_FORMAT_NONE = 0,
   FT_GLYPH_FORMAT_COMPOSITE = 1668246896,
   FT_GLYPH_FORMAT_BITMAP = 1651078259,
   FT_GLYPH_FORMAT_OUTLINE = 1869968492,
   FT_GLYPH_FORMAT_PLOTTER = 1886154612,
}

pub enum FT_LibraryRec { }
pub type FT_Library = *mut FT_LibraryRec;

pub enum FT_SubGlyphRec { }
pub type FT_SubGlyph = *mut FT_SubGlyphRec;

pub enum FT_Slot_InternalRec { }
pub type FT_Slot_Internal = *mut FT_Slot_InternalRec;

pub enum FT_Size_InternalRec { }
pub type FT_Size_Internal = *mut FT_Size_InternalRec;

pub enum FT_DriverRec { }
pub type FT_Driver = *mut FT_DriverRec;

pub enum FT_Face_InternalRec { }
pub type FT_Face_Internal = *mut FT_Face_InternalRec;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct FT_Bitmap_Size {
   pub height: FT_Short,
   pub width: FT_Short,
   pub size: FT_Pos,
   pub x_ppem: FT_Pos,
   pub y_ppem: FT_Pos,
}
impl Default for FT_Bitmap_Size {
   fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct FT_CharMapRec {
   pub face: FT_Face,
   pub encoding: FT_Encoding,
   pub platform_id: FT_UShort,
   pub encoding_id: FT_UShort,
}
impl Default for FT_CharMapRec {
   fn default() -> Self { unsafe { mem::zeroed() } }
}
pub type FT_CharMap = *mut FT_CharMapRec;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct FT_Generic {
   pub data: *mut c_void,
   pub finalizer: FT_Generic_Finalizer,
}
impl Default for FT_Generic {
   fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct FT_BBox {
   pub xMin: FT_Pos,
   pub yMin: FT_Pos,
   pub xMax: FT_Pos,
   pub yMax: FT_Pos,
}
impl Default for FT_BBox {
   fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct FT_Glyph_Metrics {
   pub width: FT_Pos,
   pub height: FT_Pos,
   pub horiBearingX: FT_Pos,
   pub horiBearingY: FT_Pos,
   pub horiAdvance: FT_Pos,
   pub vertBearingX: FT_Pos,
   pub vertBearingY: FT_Pos,
   pub vertAdvance: FT_Pos,
}
impl Default for FT_Glyph_Metrics {
   fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct FT_Bitmap {
   pub rows: c_uint,
   pub width: c_uint,
   pub pitch: c_int,
   pub buffer: *mut c_uchar,
   pub num_grays: c_ushort,
   pub pixel_mode: c_uchar,
   pub palette_mode: c_uchar,
   pub palette: *mut c_void,
}
impl Default for FT_Bitmap {
   fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct FT_GlyphSlotRec {
   pub library: FT_Library,
   pub face: FT_Face,
   pub next: FT_GlyphSlot,
   pub reserved: FT_UInt,
   pub generic: FT_Generic,
   pub metrics: FT_Glyph_Metrics,
   pub linearHoriAdvance: FT_Fixed,
   pub linearVertAdvance: FT_Fixed,
   pub advance: FT_Vector,
   pub format: FT_Glyph_Format,
   pub bitmap: FT_Bitmap,
   pub bitmap_left: FT_Int,
   pub bitmap_top: FT_Int,
   pub outline: FT_Outline,
   pub num_subglyphs: FT_UInt,
   pub subglyphs: FT_SubGlyph,
   pub control_data: *mut c_void,
   pub control_len: c_long,
   pub lsb_delta: FT_Pos,
   pub rsb_delta: FT_Pos,
   pub other: *mut c_void,
   pub internal: FT_Slot_Internal,
}
impl Default for FT_GlyphSlotRec {
   fn default() -> Self { unsafe { mem::zeroed() } }
}
pub type FT_GlyphSlot = *mut FT_GlyphSlotRec;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct FT_Size_Metrics {
   pub x_ppem: FT_UShort,
   pub y_ppem: FT_UShort,
   pub x_scale: FT_Fixed,
   pub y_scale: FT_Fixed,
   pub ascender: FT_Pos,
   pub descender: FT_Pos,
   pub height: FT_Pos,
   pub max_advance: FT_Pos,
}
impl Default for FT_Size_Metrics {
   fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct FT_SizeRec {
   pub face: FT_Face,
   pub generic: FT_Generic,
   pub metrics: FT_Size_Metrics,
   pub internal: FT_Size_Internal,
}
impl Default for FT_SizeRec {
   fn default() -> Self { unsafe { mem::zeroed() } }
}
pub type FT_Size = *mut FT_SizeRec;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct FT_MemoryRec {
   pub user: *mut c_void,
   pub alloc: FT_Alloc_Func,
   pub free: FT_Free_Func,
   pub realloc: FT_Realloc_Func,
}
impl Default for FT_MemoryRec {
   fn default() -> Self { unsafe { mem::zeroed() } }
}
pub type FT_Memory = *mut FT_MemoryRec;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct FT_StreamDesc {
   pub _bindgen_data_: [u64; 1usize],
}
impl FT_StreamDesc {
   pub unsafe fn value(&mut self) -> *mut c_long {
   let raw: *mut u8 = mem::transmute(&self._bindgen_data_);
      mem::transmute(raw.offset(0))
   }
   pub unsafe fn pointer(&mut self) -> *mut *mut c_void {
   let raw: *mut u8 = mem::transmute(&self._bindgen_data_);
      mem::transmute(raw.offset(0))
   }
}
impl Default for FT_StreamDesc {
   fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct FT_StreamRec {
   pub base: *mut c_uchar,
   pub size: c_ulong,
   pub pos: c_ulong,
   pub descriptor: FT_StreamDesc,
   pub pathname: FT_StreamDesc,
   pub read: FT_Stream_IoFunc,
   pub close: FT_Stream_CloseFunc,
   pub memory: FT_Memory,
   pub cursor: *mut c_uchar,
   pub limit: *mut c_uchar,
}
impl Default for FT_StreamRec {
   fn default() -> Self { unsafe { mem::zeroed() } }
}
pub type FT_Stream = *mut FT_StreamRec;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct FT_ListNodeRec {
   pub prev: FT_ListNode,
   pub next: FT_ListNode,
   pub data: *mut c_void,
}
impl Default for FT_ListNodeRec {
   fn default() -> Self { unsafe { mem::zeroed() } }
}
pub type FT_ListNode = *mut FT_ListNodeRec;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct FT_ListRec {
   pub head: FT_ListNode,
   pub tail: FT_ListNode,
}
impl Default for FT_ListRec {
   fn default() -> Self { unsafe { mem::zeroed() } }
}
pub type FT_List = *mut FT_ListRec;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct FT_FaceRec {
   pub num_faces: FT_Long,
   pub face_index: FT_Long,
   pub face_flags: FT_Long,
   pub style_flags: FT_Long,
   pub num_glyphs: FT_Long,
   pub family_name: *mut FT_String,
   pub style_name: *mut FT_String,
   pub num_fixed_sizes: FT_Int,
   pub available_sizes: *mut FT_Bitmap_Size,
   pub num_charmaps: FT_Int,
   pub charmaps: *mut FT_CharMap,
   pub generic: FT_Generic,
   pub bbox: FT_BBox,
   pub units_per_EM: FT_UShort,
   pub ascender: FT_Short,
   pub descender: FT_Short,
   pub height: FT_Short,
   pub max_advance_width: FT_Short,
   pub max_advance_height: FT_Short,
   pub underline_position: FT_Short,
   pub underline_thickness: FT_Short,
   pub glyph: FT_GlyphSlot,
   pub size: FT_Size,
   pub charmap: FT_CharMap,
   pub driver: FT_Driver,
   pub memory: FT_Memory,
   pub stream: FT_Stream,
   pub sizes_list: FT_ListRec,
   pub autohint: FT_Generic,
   pub extensions: *mut c_void,
   pub internal: FT_Face_Internal,
}
impl Default for FT_FaceRec {
   fn default() -> Self { unsafe { mem::zeroed() } }
}
pub type FT_Face = *mut FT_FaceRec;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct FT_Vector {
   pub x: FT_Pos,
   pub y: FT_Pos,
}
impl Default for FT_Vector {
   fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct FT_Outline {
   pub n_contours: c_short,
   pub n_points: c_short,
   pub points: *mut FT_Vector,
   pub tags: *mut c_char,
   pub contours: *mut c_short,
   pub flags: c_int,
}
impl Default for FT_Outline {
   fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct FT_Outline_Funcs {
   pub move_to: FT_Outline_MoveToFunc,
   pub line_to: FT_Outline_LineToFunc,
   pub conic_to: FT_Outline_ConicToFunc,
   pub cubic_to: FT_Outline_CubicToFunc,
   pub shift: c_int,
   pub delta: FT_Pos,
}
impl Default for FT_Outline_Funcs {
   fn default() -> Self { unsafe { mem::zeroed() } }
}


static mut FT_Outline_Decompose_Ptr:             FnPtr = NULL_PTR;
static mut FT_Init_FreeType_Ptr:                 FnPtr = NULL_PTR;
static mut FT_Done_FreeType_Ptr:                 FnPtr = NULL_PTR;
static mut FT_New_Face_Ptr:                      FnPtr = NULL_PTR;
static mut FT_Reference_Face_Ptr:                FnPtr = NULL_PTR;
static mut FT_Done_Face_Ptr:                     FnPtr = NULL_PTR;
static mut FT_Load_Char_Ptr:                     FnPtr = NULL_PTR;
static mut FT_Set_Pixel_Sizes_Ptr:               FnPtr = NULL_PTR;
static mut FT_Get_Kerning_Ptr:                   FnPtr = NULL_PTR;
static mut FT_Get_Char_Index_Ptr:                FnPtr = NULL_PTR;


#[inline]
pub unsafe fn FT_Outline_Decompose(
      outline: *mut FT_Outline,
      func_interface: *const FT_Outline_Funcs,
      user: *mut c_void
) -> FT_Error {
   mem::transmute::<_, extern "system" fn(
      *mut FT_Outline, *const FT_Outline_Funcs, *mut c_void
   ) -> FT_Error>(FT_Outline_Decompose_Ptr)(outline, func_interface, user)
}

#[inline]
pub unsafe fn FT_Init_FreeType(
   alibrary: *mut FT_Library
) -> FT_Error {
   mem::transmute::<_, extern "system" fn(
      *mut FT_Library
   ) -> FT_Error>(FT_Init_FreeType_Ptr)(alibrary)
}

#[inline]
pub unsafe fn FT_Done_FreeType(
   library: FT_Library
) -> FT_Error {
   mem::transmute::<_, extern "system" fn(
      FT_Library
   ) -> FT_Error>(FT_Done_FreeType_Ptr)(library)
}

#[inline]
pub unsafe fn FT_New_Face(
   library: FT_Library,
   filepathname: *const c_char,
   face_index: FT_Long, aface:
   *mut FT_Face
) -> FT_Error {
   mem::transmute::<_, extern "system" fn(
      FT_Library, *const c_char, FT_Long, *mut FT_Face
   ) -> FT_Error>(FT_New_Face_Ptr)(library, filepathname, face_index, aface)
}

#[inline]
pub unsafe fn FT_Reference_Face(
   face: FT_Face
) -> FT_Error {
   mem::transmute::<_, extern "system" fn(
      FT_Face
   ) -> FT_Error>(FT_Reference_Face_Ptr)(face)
}

#[inline]
pub unsafe fn FT_Done_Face(
   face: FT_Face
) -> FT_Error {
   mem::transmute::<_, extern "system" fn(
      FT_Face
   ) -> FT_Error>(FT_Done_Face_Ptr)(face)
}

#[inline]
pub unsafe fn FT_Load_Char(
   face: FT_Face,
   char_code: FT_ULong,
   load_flags: FT_Int32
) -> FT_Error {
   mem::transmute::<_, extern "system" fn(
      FT_Face, FT_ULong, FT_Int32
   ) -> FT_Error>(FT_Load_Char_Ptr)(face, char_code, load_flags)
}

#[inline]
pub unsafe fn FT_Get_Char_Index(
   face: FT_Face,
   charcode: FT_ULong
) -> FT_UInt {
   mem::transmute::<_, extern "system" fn(
      FT_Face, FT_ULong
   ) -> FT_UInt>(FT_Get_Char_Index_Ptr)(face, charcode)
}

#[inline]
pub unsafe fn FT_Set_Pixel_Sizes(
   face: FT_Face,
   pixel_width: FT_UInt,
   pixel_height: FT_UInt
) -> FT_Error {
   mem::transmute::<_, extern "system" fn(
      FT_Face, FT_UInt, FT_UInt
   ) -> FT_Error>(FT_Set_Pixel_Sizes_Ptr)(face, pixel_width, pixel_height)
}

#[inline]
pub unsafe fn FT_Get_Kerning(
   face: FT_Face,
   left_glyph: FT_UInt,
   right_glyph: FT_UInt,
   kern_mode: FT_UInt,
   akerning: *mut FT_Vector
) -> FT_Error {
   mem::transmute::<_, extern "system" fn(
      FT_Face, FT_UInt, FT_UInt, FT_UInt, *mut FT_Vector
   ) -> FT_Error>(FT_Get_Kerning_Ptr)(face, left_glyph, right_glyph, kern_mode, akerning)
}


pub unsafe fn load_functions(loader: &FnPtrLoader) -> bool {
   FT_Outline_Decompose_Ptr = loader.load("FT_Outline_Decompose");
   FT_Init_FreeType_Ptr = loader.load("FT_Init_FreeType");
   FT_Done_FreeType_Ptr = loader.load("FT_Done_FreeType");
   FT_New_Face_Ptr = loader.load("FT_New_Face");
   FT_Reference_Face_Ptr = loader.load("FT_Reference_Face");
   FT_Done_Face_Ptr = loader.load("FT_Done_Face");
   FT_Load_Char_Ptr = loader.load("FT_Load_Char");
   FT_Get_Char_Index_Ptr = loader.load("FT_Get_Char_Index");
   FT_Set_Pixel_Sizes_Ptr = loader.load("FT_Set_Pixel_Sizes");
   FT_Get_Kerning_Ptr = loader.load("FT_Get_Kerning");

   true
}

