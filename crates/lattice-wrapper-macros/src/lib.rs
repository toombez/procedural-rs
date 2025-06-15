extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, parse_macro_input, Ident, Token};

struct SizeWrapperInput {
    wrapper_name: Ident,
    inner_size: Ident,
}

impl Parse for SizeWrapperInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let wrapper_name = input.parse()?;
        input.parse::<Token![,]>()?;
        let inner_size = input.parse()?;

        Ok(Self {
            wrapper_name,
            inner_size,
        })
    }
}

#[proc_macro]
pub fn define_size_wrapper(input: TokenStream) -> TokenStream {
    let SizeWrapperInput {
        wrapper_name,
        inner_size,
    } = parse_macro_input!(input as SizeWrapperInput);

    quote! {
        #[cfg_attr(feature = "wasm", wasm_bindgen)]
        #[derive(Debug, Clone)]
        pub struct #wrapper_name(#inner_size);

        #[cfg_attr(feature = "wasm", wasm_bindgen)]
        impl #wrapper_name {
            #[cfg_attr(feature = "wasm", wasm_bindgen(constructor))]
            pub fn new(sizes: Vec<usize>) -> Self {
                Self(#inner_size::from_iter(sizes.into_iter()))
            }

            #[cfg_attr(feature = "wasm", wasm_bindgen)]
            pub fn get(&self, index: usize) -> Option<usize> {
                self.0.get(index).cloned()
            }

            #[cfg_attr(feature = "wasm", wasm_bindgen(getter))]
            pub fn sizes(&self) -> Vec<usize> {
                self.0.values().into_iter().collect()
            }
        }

        impl Into<#inner_size> for #wrapper_name {
            fn into(self) -> #inner_size {
                self.0
            }
        }

        impl From<#inner_size> for #wrapper_name {
            fn from(size: #inner_size) -> Self {
                Self(size)
            }
        }
    }
    .into()
}

struct PointWrapperInput {
    wrapper_name: Ident,
    inner_point: Ident,
}

impl Parse for PointWrapperInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let wrapper_name = input.parse()?;
        input.parse::<Token![,]>()?;
        let inner_point = input.parse()?;

        Ok(Self {
            wrapper_name,
            inner_point,
        })
    }
}

#[proc_macro]
pub fn define_point_wrapper(input: TokenStream) -> TokenStream {
    let PointWrapperInput {
        wrapper_name,
        inner_point,
    } = parse_macro_input!(input as PointWrapperInput);

    quote! {
        #[cfg_attr(feature = "wasm", wasm_bindgen)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct #wrapper_name(#inner_point);

        #[cfg_attr(feature = "wasm", wasm_bindgen)]
        impl #wrapper_name {
            #[cfg_attr(feature = "wasm", wasm_bindgen(constructor))]
            pub fn new(coords: Vec<isize>) -> Self {
                Self(#inner_point::from_iter(coords.into_iter().map(|coord| coord as i128)))
            }

            #[cfg_attr(feature = "wasm", wasm_bindgen)]
            pub fn get(&self, index: usize) -> Option<i128> {
                self.0.get(index).copied()
            }

            #[cfg_attr(feature = "wasm", wasm_bindgen(getter))]
            pub fn coords(&self) -> Vec<isize> {
                self.0.values().into_iter().map(|coord| coord as isize).collect()
            }
        }

        impl Into<#inner_point> for #wrapper_name {
            fn into(self) -> #inner_point {
                self.0
            }
        }

        impl From<#inner_point> for #wrapper_name {
            fn from(value: #inner_point) -> Self {
                Self(value)
            }
        }
    }
    .into()
}

struct LatticeWrapperInput {
    wrapper_name: Ident,
    state: Ident,
    wrapper_point: Ident,
    wrapper_size: Ident,
    automaton: Ident,
    inner_lattice: Ident,
    inner_size: Ident,
    inner_point: Ident,
}

impl Parse for LatticeWrapperInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let wrapper_name = input.parse()?;
        input.parse::<Token![,]>()?;
        let state = input.parse()?;
        input.parse::<Token![,]>()?;
        let wrapper_point = input.parse()?;
        input.parse::<Token![,]>()?;
        let wrapper_size = input.parse()?;
        input.parse::<Token![,]>()?;
        let automaton = input.parse()?;
        input.parse::<Token![,]>()?;
        let inner_lattice = input.parse()?;
        input.parse::<Token![,]>()?;
        let inner_size = input.parse()?;
        input.parse::<Token![,]>()?;
        let inner_point = input.parse()?;

        Ok(Self {
            wrapper_name,
            state,
            wrapper_point,
            wrapper_size,
            automaton,
            inner_lattice,
            inner_size,
            inner_point,
        })
    }
}

#[proc_macro]
pub fn define_lattice_wrapper(input: TokenStream) -> TokenStream {
    let LatticeWrapperInput {
        wrapper_name,
        state,
        wrapper_point,
        wrapper_size,
        automaton,
        inner_lattice,
        inner_size,
        inner_point,
    } = parse_macro_input!(input as LatticeWrapperInput);

    let tuple_name = Ident::new(
        &format!("{}PointStateTuple", wrapper_name),
        wrapper_name.span(),
    );

    quote! {
        #[cfg_attr(feature = "wasm", wasm_bindgen)]
        #[derive(Debug, Clone)]
        pub struct #wrapper_name {
            inner: #inner_lattice,
        }

        #[cfg_attr(feature = "wasm", wasm_bindgen)]
        impl #wrapper_name {
            #[cfg_attr(feature = "wasm", wasm_bindgen(constructor))]
            pub fn new(states: Vec<#state>, size: #wrapper_size) -> Self {
                Self::from_states(states, size)
            }

            #[cfg_attr(feature = "wasm", wasm_bindgen)]
            pub fn from_states(states: Vec<#state>, size: #wrapper_size) -> Self {
                Self {
                    inner: #inner_lattice::from_states(states, size.into()),
                }
            }

            #[cfg_attr(feature = "wasm", wasm_bindgen)]
            pub fn from_size(size: #wrapper_size) -> Self {
                let size: #inner_size = size.into();

                Self {
                    inner: #inner_lattice::from(size),
                }
            }

            #[cfg_attr(feature = "wasm", wasm_bindgen)]
            pub fn get_state(&self, point: &#wrapper_point) -> #state {
                self.inner.get_state(&(*point).into())
            }

            #[cfg_attr(feature = "wasm", wasm_bindgen)]
            pub fn set_state(&mut self, point: &#wrapper_point, state: #state) {
                self.inner.set_state(&(*point).into(), &state);
            }

            #[cfg_attr(feature = "wasm", wasm_bindgen(getter))]
            pub fn points(&self) -> Vec<#wrapper_point> {
                self.inner.points().into_iter().map(|point| point.into()).collect()
            }

            #[cfg_attr(feature = "wasm", wasm_bindgen(getter))]
            pub fn states(&self) -> Vec<#state> {
                self.inner.states()
            }

            #[cfg_attr(feature = "wasm", wasm_bindgen(getter))]
            pub fn points_with_states(&self) -> Vec<#tuple_name> {
                self.inner.clone().into_iter().map(|(point, state)| {
                    let point: #wrapper_point = point.into();
                    #tuple_name::from((point, state))
                }).collect()
            }

            #[cfg_attr(feature = "wasm", wasm_bindgen)]
            pub fn set_boundary_handling(&mut self, boundary_handling: BoundaryHandling) {
                self.inner.set_boundary_handling(boundary_handling);
            }

            #[cfg_attr(feature = "wasm", wasm_bindgen(getter))]
            pub fn boundary_handling(&self) -> BoundaryHandling {
                self.inner.boundary_handling()
            }

            #[cfg_attr(feature = "wasm", wasm_bindgen)]
            pub fn size(&self) -> #wrapper_size {
                self.inner.size().into()
            }

            #[cfg_attr(feature = "wasm", wasm_bindgen)]
            pub fn set_size(&mut self, size: #wrapper_size) {
                self.inner.set_size(size.into());
            }

            // #[cfg_attr(feature = "wasm", wasm_bindgen)]
            // pub fn transform_point(&self, point: &#wrapper_point) -> #wrapper_point {
            //     self.inner.transform_point(point)
            // }
        }

        impl From<#inner_lattice> for #wrapper_name {
            fn from(value: #inner_lattice) -> Self {
                Self {
                    inner: value
                }
            }
        }

        impl Into<#inner_lattice> for #wrapper_name {
            fn into(self) -> #inner_lattice {
                self.inner
            }
        }

        #[cfg_attr(feature = "wasm", wasm_bindgen)]
        impl #automaton {
            #[cfg_attr(feature = "wasm", wasm_bindgen(js_name = "step"))]
            pub fn step_wrapper(&self, inner_lattice: &mut #wrapper_name) {
                self.step(&mut inner_lattice.inner);
            }
        }

        #[cfg_attr(feature = "wasm", wasm_bindgen)]
        #[derive(Debug, Clone)]
        pub struct #tuple_name {
            pub point: #wrapper_point,
            pub state: #state,
        }

        #[cfg_attr(feature = "wasm", wasm_bindgen)]
        impl #tuple_name {
            #[cfg_attr(feature = "wasm", wasm_bindgen(constructor))]
            pub fn new(point: #wrapper_point, state: #state) -> Self {
                Self { point, state }
            }
        }

        impl From<(#wrapper_point, #state)> for #tuple_name {
            fn from(value: (#wrapper_point, #state)) -> Self {
                Self {
                    point: value.0,
                    state: value.1,
                }
            }
        }
    }
    .into()
}
