use syn::__private::{Span, ToTokens};

use crate::log::LogLevel;

/// a growable buffer of raw UTF-8 encoded bytes, allocated on the global heap
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ByteBuffer {
	pub buf: Vec<u8>,
}

impl std::fmt::Display for ByteBuffer {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", String::from_utf8_lossy(&self.buf))
	}
}

impl ByteBuffer {
	// /// Creates a new [`ByteBuffer`].
	// pub fn new() -> Self {
	// Self { buf: Vec::new() }
	// }
	//
	// Creates a new [`ByteBuffer`] with the given capacity.
	// pub fn with_capacity(capacity: usize) -> Self {
	// Self {
	// buf: Vec::with_capacity(capacity),
	// }
	// }
	//
	// pub fn as_mut(&mut self) -> &mut [u8] {
	// self.buf.as_mut()
	// }
	//
	// pub fn from_str_vec(strings: Vec<&str>) -> Self {
	// let mut buf = Vec::new();
	// for s in strings {
	// buf.extend_from_slice(s.as_bytes());
	// }
	// Self { buf }
	// }
	//
	// pub fn from_string(s: String) -> Self {
	// Self { buf: s.into_bytes() }
	// }
	//
	// pub fn from_byte_buffer(byte_buffer: Vec<u8>) -> Self {
	// Self { buf: byte_buffer }
	// }

	pub fn from_str(s: &str) -> Self {
		Self {
			buf: s.as_bytes().to_vec(),
		}
	}

	pub fn as_ref(&self) -> &[u8] {
		self.buf.as_ref()
	}

	pub fn from_bindgen_bindings(bindings: bindgen::Bindings) -> Self {
		let mut buf = Vec::new();
		bindings.write(Box::new(&mut buf)).unwrap();
		Self { buf }
	}
}

/// represents a symbol in the generated bindings
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Symbol {
	pub buf: ByteBuffer,
}

impl Symbol {
	/// creates a new item with the given name
	pub fn new(name: &str) -> Self {
		Self {
			buf: ByteBuffer::from_str(name),
		}
	}
}

/// Parses a given byte buffer of UTF-8 encoded Rust code into
/// a vector of [`Symbol`]s.
///
/// # Arguments
///
/// * `buf` - the byte buffer to parse
///
/// # Returns
///
/// * `Ok(Vec<Symbol>)` if the parsing was successful
/// * `Err(String)` otherwise
///
///
/// # Examples
///
/// ```
/// use codegen::item::parse_symbols;
///
/// let items = parse_symbols(b"pub struct Foo;").unwrap();
///
/// assert_eq!(items.len(), 1);
/// assert_eq!(items[0].buf, b"Foo");
///
/// let items = parse_symbols(b"pub struct Foo; pub struct Bar;").unwrap();
///
/// assert_eq!(items.len(), 2);
/// assert_eq!(items[0].buf, b"Foo");
/// assert_eq!(items[1].buf, b"Bar");
///
/// let items = parse_symbols(b"pub struct Foo; pub fn Bar(mom: &str) -> Foo { /* ... */}; pub enum Baz;").unwrap();
///
/// assert_eq!(items.len(), 3);
///
/// assert_eq!(items[0].buf, b"Foo");
/// assert_eq!(items[1].buf, b"Bar");
/// assert_eq!(items[2].buf, b"Baz");
/// ```
pub fn parse_symbols(buf: ByteBuffer) -> Result<Vec<Symbol>, String> {
	let mut symbols: Vec<Symbol> = Vec::new();

	let file = syn::parse_file(&String::from_utf8(buf.buf).unwrap_or_else(|_| {
		log!(LogLevel::Error, "buffer is not valid UTF-8");
		panic!("ERROR: buffer is not valid UTF-8");
	}))
	.unwrap_or_else(|e| {
		log!(LogLevel::Error, format!("failed to parse buffer: {}", e));
		panic!("ERROR: failed to parse buffer: {}", e);
	});

	for item in file.items {
		match item {
			syn::Item::Struct(s) => symbols.push(Symbol::new(&s.ident.to_string())),
			syn::Item::Enum(e) => symbols.push(Symbol::new(&e.ident.to_string())),
			syn::Item::Fn(f) => symbols.push(Symbol::new(&f.sig.ident.to_string())),
			syn::Item::Const(c) => symbols.push(Symbol::new(&c.ident.to_string())),
			syn::Item::Union(u) => symbols.push(Symbol::new(&u.ident.to_string())),
			syn::Item::Static(s) => symbols.push(Symbol::new(&s.ident.to_string())),
			syn::Item::Type(t) => symbols.push(Symbol::new(&t.ident.to_string())),
			_ => (),
		}
	}

	Ok(symbols)
}

/// converts a Vec<[`Symbol`]> into a Vec<[`String`]>
///
/// # Arguments
///
/// * `symbols` - the [`Vec<Symbol>`] to convert
///
/// # Returns
///
/// * `Vec<String>`
pub fn generate_features(symbols: Vec<Symbol>) -> Vec<String> {
	let mut features = Vec::new();

	for symbol in symbols.iter() {
		let symbol = String::from_utf8_lossy(symbol.buf.as_ref());
		let symbol = symbol.to_string();

		// if symbol does NOT contain "Dart_", skip it
		if !symbol.contains("Dart_") {
			continue;
		}

		// if symbol DOES contain "_Dart_", skip it
		if symbol.contains("_Dart_") {
			continue;
		}

		// push feature to features vector
		features.push(symbol);
	}

	features
}

/// Adds a feature macro to a given [`ByteBuffer`] for every occurence of a given feature name.
///
/// `cfg()` macros will be added for _EVERY_ occurence of the feature name, ie. the Item definition
/// as well as any other item that references the feature name.
///
/// # Arguments
///
/// * `buf` - mutable [`ByteBuffer`] to add the feature macro to
/// * `feature` - [`String`] containing the feature name
///
/// # Returns
///
/// * `Ok(())` if the feature macro was added successfully
/// * `Err(String)` otherwise
///
/// # Examples
///
/// ```
/// use codegen::symbols::add_feature_macro;
///
/// let mut buf = b"pub struct Foo;".to_vec();
///
/// add_feature_macro(&mut buf, "Foo".to_string()).unwrap();
///
/// assert_eq!(buf, b"#[cfg(feature = \"Foo\")]\npub struct Foo;".to_vec());
///
/// let mut buf = b"pub struct Foo; pub fn mom() {}".to_vec();
///
/// add_feature_macro(&mut buf, "mom".to_string()).unwrap();
///
/// assert_eq!(buf, b"pub struct Foo; #[cfg(feature = \"mom\")]\npub fn mom() {}".to_vec());
///
/// add_feature_macro(&mut buf, "Foo".to_string()).unwrap();
///
/// assert_eq!(buf, b"#[cfg(feature = \"Foo\")]\npub struct Foo; #[cfg(feature = \"mom\")]\npub fn mom() {}".to_vec());
///
/// let mut buf = b"pub struct Foo; pub struct Baz { id: Foo, mother: u8 }; pub fn mom() {}".to_vec();
///
/// add_feature_macro(&mut buf, "Foo".to_string()).unwrap();
///
///
/// assert_eq!(buf, b"#[cfg(feature = \"Foo\")]\npub struct Foo; pub struct Baz { #[cfg(feature = \"Foo\")]\nid: Foo, mother: u8 }; pub fn mom() {}".to_vec());
/// ```
pub fn add_feature_macro(buf: &mut ByteBuffer, feature: String) -> Result<(), String> {
	// log!(format!("adding feature macro for feature: {}", feature));

	let mut file = syn::parse_file(&String::from_utf8(buf.buf.clone()).unwrap_or_else(|_| {
		log!(LogLevel::Error, "buffer is not valid UTF-8");
		panic!("ERROR: buffer is not valid UTF-8");
	}))
	.unwrap_or_else(|e| {
		log!(LogLevel::Error, format!("failed to parse buffer: {}", e));
		panic!("ERROR: failed to parse buffer: {}", e);
	});

	let mut found_symbol_definition = false;

	// iterate over all items in the file, seaching for the symbol
	for item in file.items.iter_mut() {
		match item {
			// if item is a Struct
			syn::Item::Struct(s) => {
				// check if the struct name matches the feature name
				if s.ident == feature {
					s.attrs.insert(0, syn::parse_quote!(#[cfg(feature = #feature)]));
					found_symbol_definition = true;
				} else {
					// check if the struct field types contain the feature name
					for field in s.fields.iter_mut() {
						if let syn::Type::Path(p) = &mut field.ty {
							if p.path.segments[0].ident == feature {
								field.attrs.insert(0, syn::parse_quote!(#[cfg(feature = #feature)]));
							}
						}
					}
				}
			},

			// if item is an Enum
			syn::Item::Enum(e) => {
				// check if the enum name matches the feature name
				if e.ident == feature {
					e.attrs.insert(0, syn::parse_quote!(#[cfg(feature = #feature)]));
					found_symbol_definition = true;
				} else {
					// check if the enum variant types contain the feature name
					for variant in e.variants.iter_mut() {
						if let syn::Fields::Named(n) = &mut variant.fields {
							for field in n.named.iter_mut() {
								if let syn::Type::Path(p) = &mut field.ty {
									if p.path.segments[0].ident == feature {
										e.attrs.insert(0, syn::parse_quote!(#[cfg(feature = #feature)]));
									}
								}
							}
						}
					}
				}
			},

			// if item is a Function
			syn::Item::Fn(f) => {
				// check if the function signature matches the feature name
				if f.sig.ident == feature {
					f.attrs.insert(0, syn::parse_quote!(#[cfg(feature = #feature)]));
					found_symbol_definition = true;
				} else {
					// check if the function arguments contain the feature name, or a mutable reference to the feature
					// name

					for arg in f.sig.inputs.iter_mut() {
						if let syn::FnArg::Typed(t) = arg {
							if let syn::Type::Path(p) = &mut *t.ty {
								if p.path.segments[0].ident == feature {
									f.attrs.insert(0, syn::parse_quote!(#[cfg(feature = #feature)]));
								}
							}
						}
					}

					// check if the function return type contains the feature name
					if let syn::ReturnType::Type(_, t) = &f.sig.output {
						if let syn::Type::Path(p) = &**t {
							if p.path.segments.last().unwrap().ident == syn::Ident::new(&feature, Span::call_site()) {
								f.attrs.insert(0, syn::parse_quote!(#[cfg(feature = #feature)]));
							}
						}
					}
				}

				// generated functions will not have a funtion body.
			},

			// if item is a Constant
			syn::Item::Const(c) => {
				// check if the constant name matches the feature name
				if c.ident == feature {
					c.attrs.insert(0, syn::parse_quote!(#[cfg(feature = #feature)]));
					found_symbol_definition = true;
				} else {
					// check if the constant type contains the feature name
					if let syn::Type::Path(p) = &*c.ty {
						if p.path.segments.last().unwrap().ident == syn::Ident::new(&feature, Span::call_site()) {
							c.attrs.insert(0, syn::parse_quote!(#[cfg(feature = #feature)]));
						}
					}
				}
			},

			// if item is a Union
			syn::Item::Union(u) => {
				// check if the union name matches the feature name
				if u.ident == feature {
					u.attrs.insert(0, syn::parse_quote!(#[cfg(feature = #feature)]));
					found_symbol_definition = true;
				} else {
					// check if the union field types contain the feature name
					for field in u.fields.named.iter_mut() {
						if let syn::Type::Path(p) = &field.ty {
							if p.path.segments.last().unwrap().ident == syn::Ident::new(&feature, Span::call_site()) {
								u.attrs.insert(0, syn::parse_quote!(#[cfg(feature = #feature)]));
							}
						}
					}
				}
			},

			// if item is a Static
			syn::Item::Static(s) => {
				// check if the static name matches the feature name
				if s.ident == feature {
					s.attrs.insert(0, syn::parse_quote!(#[cfg(feature = #feature)]));
					found_symbol_definition = true;
				} else {
					// check if the static type contains the feature name
					if let syn::Type::Path(p) = &*s.ty {
						if p.path.segments.last().unwrap().ident == syn::Ident::new(&feature, Span::call_site()) {
							s.attrs.insert(0, syn::parse_quote!(#[cfg(feature = #feature)]));
						}
					}
				}
			},

			// if item is a Type
			syn::Item::Type(t) => {
				// check if the type name matches the feature name
				if t.ident == feature {
					t.attrs.insert(0, syn::parse_quote!(#[cfg(feature = #feature)]));
					found_symbol_definition = true;
				} else {
					// check if the type definition contains the feature name
					if let syn::Type::Path(p) = &*t.ty {
						if p.path.segments.last().unwrap().ident == syn::Ident::new(&feature, Span::call_site()) {
							t.attrs.insert(0, syn::parse_quote!(#[cfg(feature = #feature)]));
						}
					}
				}
			},

			// if item in a foreign interface block
			syn::Item::ForeignMod(f) => {
				for item in f.items.iter_mut() {
					// if item is a foreign function
					if let syn::ForeignItem::Fn(f) = item {
						// check if the foreign function contains the feature name
						if f.sig.ident == feature {
							f.attrs.insert(0, syn::parse_quote!(#[cfg(feature = #feature)]));
							found_symbol_definition = true;
						} else {
							// check if the foreign function arguments contain the feature name, or a mutable pointer to
							// the feature name
							//
							// ie. `fn foo(bar: *mut FeatureName)` or `fn foo(bar: FeatureName)`

							// iterate over the foreign function arguments
							for arg in f.sig.inputs.iter_mut() {
								if let syn::FnArg::Typed(t) = arg {
									// if the argument is a mutable pointer
									if let syn::Type::Ptr(p) = &*t.ty {
										// if the pointer type is a path
										if let syn::Type::Path(p) = &*p.elem {
											// if the path is the feature name
											if p.path.segments.last().unwrap().ident ==
												syn::Ident::new(&feature, Span::call_site())
											{
												f.attrs.insert(0, syn::parse_quote!(#[cfg(feature = #feature)]));
											}
										}
									} else if let syn::Type::Path(p) = &*t.ty {
										// else if argument type is a direct reference

										// if the path is the feature name
										if p.path.segments.last().unwrap().ident ==
											syn::Ident::new(&feature, Span::call_site())
										{
											f.attrs.insert(0, syn::parse_quote!(#[cfg(feature = #feature)]));
										}
									}
								}
							}

							// check if the foreign function return type contains the feature name
							if let syn::ReturnType::Type(_, t) = &f.sig.output {
								if let syn::Type::Path(p) = &**t {
									if p.path.segments.last().unwrap().ident ==
										syn::Ident::new(&feature, Span::call_site())
									{
										f.attrs.insert(0, syn::parse_quote!(#[cfg(feature = #feature)]));
									}
								}
							}
						}
					}

					// if item is a foreign static
					if let syn::ForeignItem::Static(s) = item {
						// check if the foreign static contains the feature name
						if s.ident == feature {
							s.attrs.insert(0, syn::parse_quote!(#[cfg(feature = #feature)]));
							found_symbol_definition = true;
						} else {
							// check if the foreign static type contains the feature name
							if let syn::Type::Path(p) = &*s.ty {
								if p.path.segments.last().unwrap().ident == syn::Ident::new(&feature, Span::call_site())
								{
									s.attrs.insert(0, syn::parse_quote!(#[cfg(feature = #feature)]));
								}
							}
						}
					}
				}
			},

			// otherwise, ignore without error
			_ => (),
		}
	}

	if !found_symbol_definition {
		log!(
			LogLevel::Error,
			format!("failed to find symbol definition for {}", feature)
		);
		return Err(format!("failed to find symbol definition for {}", feature));
	}

	// write the file back to the buffer

	let mut new_buf = Vec::new();

	new_buf.extend(file.to_token_stream().to_string().into_bytes());

	buf.buf = new_buf;

	Ok(())
}
