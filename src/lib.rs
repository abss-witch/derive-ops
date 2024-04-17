use proc_macro2::{self, TokenStream};
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, spanned::Spanned, Data, DeriveInput, Fields};
///component wise addition derive.
#[proc_macro_derive(CmpAdd)]
pub fn cmp_add(item: proc_macro::TokenStream) -> proc_macro::TokenStream{
    let input = parse_macro_input!(item as DeriveInput);
    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let sum = cmp_action(input.data, quote! {+});
    proc_macro::TokenStream::from(quote! {
        impl #impl_generics std::ops::Add for #name #ty_generics #where_clause{
            fn add(self, rhs: Self) -> Self::Output{
                #sum
            }
            type Output = Self;
        }
    })
}
///component wise addition assign derive.
#[proc_macro_derive(CmpAddAssign)]
pub fn cmp_add_assign(item: proc_macro::TokenStream) -> proc_macro::TokenStream{
    let input = parse_macro_input!(item as DeriveInput);
    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let sum = cmp_action(input.data, quote! {+});
    proc_macro::TokenStream::from(quote! {
        impl #impl_generics std::ops::AddAssign for #name #ty_generics #where_clause{
            fn add_assign(&mut self, rhs: Self){
                *self = #sum
            }
        }
    })
}
///component wise subtraction derive.
#[proc_macro_derive(CmpSub)]
pub fn cmp_sub(item: proc_macro::TokenStream) -> proc_macro::TokenStream{
    let input = parse_macro_input!(item as DeriveInput);
    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let sum = cmp_action(input.data, quote! {-});
    proc_macro::TokenStream::from(quote! {
        impl #impl_generics std::ops::Sub for #name #ty_generics #where_clause{
            fn sub(self, rhs: Self) -> Self::Output{
                #sum
            }
            type Output = Self;
        }
    })
}
///component wise subtraction assign derive.
#[proc_macro_derive(CmpSubAssign)]
pub fn cmp_sub_assign(item: proc_macro::TokenStream) -> proc_macro::TokenStream{
    let input = parse_macro_input!(item as DeriveInput);
    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let sum = cmp_action(input.data, quote! {-});
    proc_macro::TokenStream::from(quote! {
        impl #impl_generics std::ops::SubAssign for #name #ty_generics #where_clause{
            fn sub_assign(&mut self, rhs: Self){
                *self = #sum
            }
        }
    })
}
///component wise multiplication derive.
#[proc_macro_derive(CmpMul)]
pub fn cmp_mul(item: proc_macro::TokenStream) -> proc_macro::TokenStream{
    let input = parse_macro_input!(item as DeriveInput);
    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let sum = cmp_action(input.data, quote! {*});
    proc_macro::TokenStream::from(quote! {
        impl #impl_generics std::ops::Mul for #name #ty_generics #where_clause{
            fn mul(self, rhs: Self) -> Self::Output{
                #sum
            }
            type Output = Self;
        }
    })
}
///component wise multiplication assign derive.
#[proc_macro_derive(CmpMulAssign)]
pub fn cmp_mul_assign(item: proc_macro::TokenStream) -> proc_macro::TokenStream{
    let input = parse_macro_input!(item as DeriveInput);
    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let sum = cmp_action(input.data, quote! {*});
    proc_macro::TokenStream::from(quote! {
        impl #impl_generics std::ops::MulAssign for #name #ty_generics #where_clause{
            fn mul_assign(&mut self, rhs: Self){
                *self = #sum
            }
        }
    })
}
///component wise division derive.
#[proc_macro_derive(CmpDiv)]
pub fn cmp_div(item: proc_macro::TokenStream) -> proc_macro::TokenStream{
    let input = parse_macro_input!(item as DeriveInput);
    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    
    let sum = cmp_action(input.data, quote! {/});
    proc_macro::TokenStream::from(quote! {
        impl #impl_generics std::ops::Div for #name #ty_generics #where_clause{
            fn div(self, rhs: Self) -> Self::Output{
                #sum
            }
            type Output = Self;
        }
    })
}
///component wise division assign derive.
#[proc_macro_derive(CmpDivAssign)]
pub fn cmp_div_assign(item: proc_macro::TokenStream) -> proc_macro::TokenStream{
    let input = parse_macro_input!(item as DeriveInput);
    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    
    let sum = cmp_action(input.data, quote! {/});
    proc_macro::TokenStream::from(quote! {
        impl #impl_generics std::ops::DivAssign for #name #ty_generics #where_clause{
            fn div_assign(&mut self, rhs: Self){
                *self = #sum
            }
        }
    })
}
///component wise remainder derive.
#[proc_macro_derive(CmpRem)]
pub fn cmp_rem(item: proc_macro::TokenStream) -> proc_macro::TokenStream{
    let input = parse_macro_input!(item as DeriveInput);
    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    
    let sum = cmp_action(input.data, quote! {%});
    proc_macro::TokenStream::from(quote! {
        impl #impl_generics std::ops::Rem for #name #ty_generics #where_clause{
            fn rem(self, rhs: Self) -> Self::Output{
                #sum
            }
            type Output = Self;
        }
    })
}
///component wise remainder assign derive.
#[proc_macro_derive(CmpRemAssign)]
pub fn cmp_rem_assign(item: proc_macro::TokenStream) -> proc_macro::TokenStream{
    let input = parse_macro_input!(item as DeriveInput);
    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    
    let sum = cmp_action(input.data, quote! {%});
    proc_macro::TokenStream::from(quote! {
        impl #impl_generics std::ops::RemAssign for #name #ty_generics #where_clause{
            fn rem_assign(&mut self, rhs: Self){
                *self = #sum
            }
        }
    })
}
///component wise negative derive.
#[proc_macro_derive(CmpNeg)]
pub fn cmp_neg(item: proc_macro::TokenStream) -> proc_macro::TokenStream{
    let input = parse_macro_input!(item as DeriveInput);
    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    
    let Data::Struct(ref data) = input.data else { unimplemented!() };
    let Fields::Named(ref fields) = data.fields else { unimplemented!() };
    let recursive = fields.named.iter().map(|f|{
        let name = &f.ident;
        quote_spanned! { f.span() => #name: -self.#name, }
    });
    let sum = quote! {
        Self { #(#recursive)* }
    };
    proc_macro::TokenStream::from(quote! {
        impl #impl_generics std::ops::Neg for #name #ty_generics #where_clause{
            type Output = Self;
            fn neg(self) -> Self::Output{
                #sum
            }
        }
    })
}
fn cmp_action(data: Data, action: TokenStream) -> TokenStream{
    let Data::Struct(ref data) = data else { unimplemented!() };
    let Fields::Named(ref fields) = data.fields else { unimplemented!() };
    let recursive = fields.named.iter().map(|f|{
        let name = &f.ident;
        quote_spanned! { f.span() => #name: self.#name #action rhs.#name, }
    });
    //expands to `Self { v1: self.v1 #action rhs.v1, v2: self.v2 #action rhs.v2, .. }`
    quote! {
        Self { #(#recursive)* }
    }
}

#[proc_macro_derive(CmpOps)]
pub fn cmp_ops(item: proc_macro::TokenStream) -> proc_macro::TokenStream{
    let mut result: proc_macro::TokenStream = quote! {}.into();
    result.extend(cmp_add(item.clone()));
    result.extend(cmp_add_assign(item.clone()));
    result.extend(cmp_sub(item.clone()));
    result.extend(cmp_sub_assign(item.clone()));
    result.extend(cmp_mul(item.clone()));
    result.extend(cmp_mul_assign(item.clone()));
    result.extend(cmp_div(item.clone()));
    result.extend(cmp_div_assign(item.clone()));
    result.extend(cmp_rem(item.clone()));
    result.extend(cmp_rem_assign(item.clone()));
    result.extend(cmp_neg(item));
    result
}