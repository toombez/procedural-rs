extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, parse_macro_input, Ident, Token};

struct LatticeWrapperInput {
    wrapper_name: Ident,
    point: Ident,
    state: Ident,
    size: Ident,
    lattice: Ident,
    automaton: Ident,
}

impl Parse for LatticeWrapperInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let wrapper_name = input.parse()?;
        input.parse::<Token![,]>()?;
        let point = input.parse()?;
        input.parse::<Token![,]>()?;
        let state = input.parse()?;
        input.parse::<Token![,]>()?;
        let size = input.parse()?;
        input.parse::<Token![,]>()?;
        let lattice = input.parse()?;
        input.parse::<Token![,]>()?;
        let automaton = input.parse()?;

        Ok(Self {
            wrapper_name,
            point,
            state,
            size,
            lattice,
            automaton,
        })
    }
}

#[proc_macro]
pub fn define_lattice_wrapper(input: TokenStream) -> TokenStream {
    let LatticeWrapperInput {
        wrapper_name,
        point,
        state,
        size,
        lattice,
        automaton,
    } = parse_macro_input!(input as LatticeWrapperInput);

    let tuple_name = Ident::new(
        &format!("{}PointStateTuple", wrapper_name),
        wrapper_name.span(),
    );

    quote! {
        #[cfg_attr(feature = "wasm", wasm_bindgen)]
        #[derive(Debug, Clone)]
        pub struct #wrapper_name {
            inner: #lattice,
        }

        #[cfg_attr(feature = "wasm", wasm_bindgen)]
        impl #wrapper_name {
            #[cfg_attr(feature = "wasm", wasm_bindgen(constructor))]
            pub fn new(states: Vec<#state>, size: #size) -> Self {
                Self::from_states(states, size)
            }


            #[cfg_attr(feature = "wasm", wasm_bindgen)]
            pub fn from_states(states: Vec<#state>, size: #size) -> Self {
                Self {
                    inner: #lattice::from_states(states, size),
                }
            }

            #[cfg_attr(feature = "wasm", wasm_bindgen)]
            pub fn from_size(size: #size) -> Self {
                Self {
                    inner: #lattice::from(size),
                }
            }

            #[cfg_attr(feature = "wasm", wasm_bindgen)]
            pub fn get_state(&self, point: &#point) -> #state {
                self.inner.get_state(point)
            }

            #[cfg_attr(feature = "wasm", wasm_bindgen)]
            pub fn set_state(&mut self, point: &#point, state: #state) {
                self.inner.set_state(point, &state);
            }

            #[cfg_attr(feature = "wasm", wasm_bindgen(getter))]
            pub fn points(&self) -> Vec<#point> {
                self.inner.points()
            }

            #[cfg_attr(feature = "wasm", wasm_bindgen(getter))]
            pub fn states(&self) -> Vec<#state> {
                self.inner.states()
            }

            #[cfg_attr(feature = "wasm", wasm_bindgen(getter))]
            pub fn points_with_states(&self) -> Vec<#tuple_name> {
                self.inner.clone().into_iter().map(#tuple_name::from).collect()
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
            pub fn size(&self) -> #size {
                self.inner.size()
            }

            #[cfg_attr(feature = "wasm", wasm_bindgen)]
            pub fn set_size(&mut self, size: #size) {
                self.inner.set_size(size);
            }

            #[cfg_attr(feature = "wasm", wasm_bindgen)]
            pub fn transform_point(&self, point: &#point) -> #point {
                self.inner.transform_point(point)
            }
        }

        impl From<#lattice> for #wrapper_name {
            fn from(value: #lattice) -> Self {
                Self {
                    inner: value
                }
            }
        }

        impl Into<#lattice> for #wrapper_name {
            fn into(self) -> #lattice {
                self.inner
            }
        }

        #[cfg_attr(feature = "wasm", wasm_bindgen)]
        impl #automaton {
            #[cfg_attr(feature = "wasm", wasm_bindgen(js_name = "step"))]
            pub fn step_wrapper(&self, lattice: &mut #wrapper_name) {
                self.step(&mut lattice.inner);
            }
        }

        #[cfg_attr(feature = "wasm", wasm_bindgen)]
        #[derive(Debug, Clone)]
        pub struct #tuple_name {
            pub point: #point,
            pub state: #state,
        }

        #[cfg_attr(feature = "wasm", wasm_bindgen)]
        impl #tuple_name {
            #[cfg_attr(feature = "wasm", wasm_bindgen(constructor))]
            pub fn new(point: #point, state: #state) -> Self {
                Self { point, state }
            }
        }

        impl From<(#point, #state)> for #tuple_name {
            fn from(value: (#point, #state)) -> Self {
                Self {
                    point: value.0,
                    state: value.1,
                }
            }
        }
    }
    .into()
}
