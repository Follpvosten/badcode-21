extern crate proc_macro;
use proc_macro::TokenStream;

use quote::quote;

fn gimme_alphabet() -> Vec<char> {
    ('a' as u8..='z' as u8).map(|v| v as char).collect()
}

#[proc_macro]
pub fn generate_valid_inputs(_: TokenStream) -> TokenStream {
    let inputs: Vec<_> = (3..=26)
        .map(|n| {
            let pi = (0..=(26 - n)).map(|i| {
                let abc = gimme_alphabet();
                let wr: Vec<_> = (&abc[i..n + i]).iter().copied().collect();
                let o: Vec<_> = (1..n - 1)
                    .map(|n| {
                        let mut v = wr.clone();
                        let f = format!("{} is missing", v.remove(n));
                        quote! {
                            (vec![#(#v),*], #f)
                        }
                    })
                    .collect();
                quote! {
                    #(#o),*
                }
            });
            quote! {
                #(#pi),*
            }
        })
        .collect();
    TokenStream::from(quote! {
        [
            #(#inputs),*
        ]
        .iter()
        .cloned()
        .collect()
    })
}

#[cfg(test)]
mod tests {
    use super::gimme_alphabet;
    #[test]
    fn correct_alphabet() {
        assert_eq!(
            gimme_alphabet(),
            vec![
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
                'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'
            ]
        )
    }
}
