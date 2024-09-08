use crate::context_handle::PtrWrap;
use crate::error::Error;
use crate::functions::*;
use crate::{
    AsRaw, AsRawMut, ContextHandle, ContextHandling, ContextInteractions, GResult, Geom
};
use geos_sys::*;
use std::sync::Arc;

/// The `GeoJSONWriter` type is used to generate `GeoJSON` formatted output from [`Geometry`](crate::Geometry).
///
/// # Example
///
/// ```
/// use geos::{Geometry, GeoJSONWriter};
///
/// let point_geom = Geometry::new_from_wkt("POINT (2.5 2.5)").expect("Invalid geometry");
/// let mut writer = GeoJSONWriter::new().expect("Failed to create GeoJSONWriter");
///
/// assert_eq!(writer.write(&point_geom).unwrap(), r#"{"type":"Point","coordinates":[2.5, 2.5]}"#);
/// ```
pub struct GeoJSONWriter {
    ptr: PtrWrap<*mut GEOSGeoJSONWriter>,
    context: Arc<ContextHandle>,
}

impl GeoJSONWriter {
    /// Creates a new `GeoJSONWriter` instance.
    ///
    /// # Example
    ///
    /// ```
    /// use geos::{Geometry, GeoJSONWriter};
    ///
    /// let point_geom = Geometry::new_from_wkt("POINT (2.5 2.5)").expect("Invalid geometry");
    /// let mut writer = GeoJSONWriter::new().expect("Failed to create GeoJSONWriter");
    ///
    /// assert_eq!(writer.write(&point_geom).unwrap(), r#"{"type":"Point","coordinates":[2.5, 2.5]}"#);
    /// ```
    pub fn new() -> GResult<GeoJSONWriter> {
        match ContextHandle::init_e(Some("GeoJSONWriter::new")) {
            Ok(context_handle) => Self::new_with_context(Arc::new(context_handle)),
            Err(e) => Err(e),
        }
    }

    /// Creates a new `GeoJSONWriter` instance with a given context.
    ///
    /// # Example
    ///
    /// ```
    /// use geos::{ContextHandling, Geometry, GeoJSONWriter};
    ///
    /// let point_geom = Geometry::new_from_wkt("POINT (2.5 2.5)").expect("Invalid geometry");
    /// let mut writer = GeoJSONWriter::new_with_context(point_geom.clone_context())
    ///                            .expect("Failed to create GeoJSONWriter");
    ///
    /// assert_eq!(writer.write(&point_geom).unwrap(), r#"{"type":"Point","coordinates":[2.5, 2.5]}");
    /// ```
    pub fn new_with_context(context: Arc<ContextHandle>) -> GResult<GeoJSONWriter> {
        unsafe {
            let ptr = GEOSGeoJSONWriter_create_r(context.as_raw());
            GeoJSONWriter::new_from_raw(ptr, context, "new_with_context")
        }
    }

    pub(crate) unsafe fn new_from_raw(
        ptr: *mut GEOSGeoJSONWriter,
        context: Arc<ContextHandle>,
        caller: &str,
    ) -> GResult<GeoJSONWriter> {
        if ptr.is_null() {
            let extra = if let Some(x) = context.get_last_error() {
                format!("\nLast error: {x}")
            } else {
                String::new()
            };
            return Err(Error::NoConstructionFromNullPtr(format!(
                "GeoJSONWriter::{caller}{extra}",
            )));
        }
        Ok(GeoJSONWriter {
            ptr: PtrWrap(ptr),
            context,
        })
    }

    /// Writes out the given `geometry` as GeoJSON format.
    ///
    /// # Example
    ///
    /// ```
    /// use geos::{Geometry, GeoJSONWriter};
    ///
    /// let point_geom = Geometry::new_from_wkt("POINT (2.5 2.5)").expect("Invalid geometry");
    /// let mut writer = GeoJSONWriter::new().expect("Failed to create GeoJSONWriter");
    ///
    /// assert_eq!(writer.write(&point_geom).unwrap(), r#"{"type":"Point","coordinates":[2.5, 2.5]}");
    /// ```
    pub fn write<G: Geom>(&mut self, geometry: &G, indent: i32) -> GResult<String> {
        unsafe {
            let ptr =
                GEOSGeoJSONWriter_writeGeometry_r(self.get_raw_context(), self.as_raw_mut(), geometry.as_raw(), indent);
            managed_string(ptr, self.get_context_handle(), "GeoJSONWriter::write")
        }
    }
}

unsafe impl Send for GeoJSONWriter {}
unsafe impl Sync for GeoJSONWriter {}

impl Drop for GeoJSONWriter {
    fn drop(&mut self) {
        unsafe { GEOSGeoJSONWriter_destroy_r(self.get_raw_context(), self.as_raw_mut()) };
    }
}

impl ContextInteractions for GeoJSONWriter {
    /// Set the context handle to the `GeoJSONWriter`.
    ///
    /// ```
    /// use geos::{ContextInteractions, ContextHandle, GeoJSONWriter};
    ///
    /// let context_handle = ContextHandle::init().expect("invalid init");
    /// let mut writer = GeoJSONWriter::new().expect("failed to create GeoJSON writer");
    /// context_handle.set_notice_message_handler(Some(Box::new(|s| println!("new message: {}", s))));
    /// writer.set_context_handle(context_handle);
    /// ```
    fn set_context_handle(&mut self, context: ContextHandle) {
        self.context = Arc::new(context);
    }

    /// Get the context handle of the `GeoJSONWriter`.
    ///
    /// ```
    /// use geos::{ContextInteractions, GeoJSONWriter};
    ///
    /// let mut writer = GeoJSONWriter::new().expect("failed to create GeoJSON writer");
    /// let context = writer.get_context_handle();
    /// context.set_notice_message_handler(Some(Box::new(|s| println!("new message: {}", s))));
    /// ```
    fn get_context_handle(&self) -> &ContextHandle {
        &self.context
    }
}

impl AsRaw for GeoJSONWriter {
    type RawType = GEOSGeoJSONWriter;

    fn as_raw(&self) -> *const Self::RawType {
        *self.ptr
    }
}

impl AsRawMut for GeoJSONWriter {
    type RawType = GEOSGeoJSONWriter;

    unsafe fn as_raw_mut_override(&self) -> *mut Self::RawType {
        *self.ptr
    }
}

impl ContextHandling for GeoJSONWriter {
    type Context = Arc<ContextHandle>;

    fn get_raw_context(&self) -> GEOSContextHandle_t {
        self.context.as_raw()
    }

    fn clone_context(&self) -> Arc<ContextHandle> {
        Arc::clone(&self.context)
    }
}
