extern crate syn;

use proc_macro::{self, TokenStream};

use quote::quote;
use syn::{Token, DeriveInput, Ident, LitStr};
use syn::parse::{ParseBuffer, ParseStream};

struct RepoOpts {
    table_name: LitStr,
    entity_type: Ident,
    id_type: Ident
}

impl syn::parse::Parse for RepoOpts {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content: ParseBuffer;
        syn::parenthesized!(content in input);

        let table_name = content.parse()?;

        content.parse::<Token![,]>()?;
        let entity_type = content.parse()?;

        content.parse::<Token![,]>()?;
        let id_type = content.parse()?;

        Ok(RepoOpts { table_name, entity_type, id_type })
    }
}

#[proc_macro_derive(Repository, attributes(repository))]
pub fn derive_repository(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input);

    let DeriveInput { ident, attrs, .. } = input;

    let repo_attr = attrs.iter()
        .filter(|attr| attr.path.segments.len() == 1 && attr.path.segments[0].ident == "repository")
        .nth(0)
        .expect("Missing attribute \"repository\" for deriving the Repository trait");

    let opts: RepoOpts = syn::parse(repo_attr.tokens.clone().into()).expect("Invalid repository attribute");

    let table_name: String = opts.table_name.value();
    let entity_type = opts.entity_type;
    let id_type = opts.id_type;

    let q_find_by_id = format!("SELECT * FROM {} WHERE id = $1 AND status <> -1", table_name);

    let output = quote! {
        #[async_trait]
        impl Repository< #entity_type, #id_type > for #ident {
            async fn select_row(&self, query: &str, args: &[&(dyn ToSql + Sync)]) -> Result<Option<Row>, DbError> {
                let client = self.conn.get().await?;
                let stmt = client.prepare_cached(query).await?;
                client.query_opt(&stmt, args).await.map_err(DbError::from)
            }

            async fn select_one(&self, query: &str, args: &[&(dyn ToSql + Sync)]) -> Result<Option<#entity_type>, DbError> {
                let result = self.select_row(query, args).await?;
                match result {
                    None => Ok(None),
                    Some(row) => {
                        #entity_type::from_row(row)
                        .map(|entity: #entity_type| Some(entity))
                        .map_err(DbError::from)
                    }
                }
            }

            async fn find_by_id(&self, id: i64) -> Result<Option<#entity_type>, DbError> {
                self.select_one(#q_find_by_id, &[&id]).await
            }

            async fn exists(&self, query: &str, args: &[&(dyn ToSql + Sync)]) -> Result<bool, DbError> {
                let result = self.select_row(query, args).await?;
                Ok(result.is_some())
            }
        }
    };

    output.into()
}
