use std::collections::HashSet;

use lazy_static::lazy_static;
use swc_core::ecma::{
    ast::{Decl, ExportDecl, Expr, ExprOrSpread, ExprStmt, Lit, ModuleItem, Pat, Stmt, Str, NamedExport},
    visit::Visit,
};

use crate::ExportInfo;

lazy_static! {
    static ref EXPORTS_SET: HashSet<&'static str> = HashSet::from([
        "getStaticProps",
        "getServerSideProps",
        "generateImageMetadata",
        "generateSitemaps",
        "generateStaticParams",
    ]);
}

pub(crate) struct CollectExportsVisitor {
    pub export_info: ExportInfo,
}

impl CollectExportsVisitor {
    pub fn new() -> Self {
        Self {
            export_info: Default::default(),
        }
    }
}

impl Visit for CollectExportsVisitor {
    fn visit_module_items(&mut self, stmts: &[swc_core::ecma::ast::ModuleItem]) {
        for stmt in stmts {
            match stmt {
                ModuleItem::Stmt(Stmt::Expr(ExprStmt {
                    expr: box Expr::Lit(Lit::Str(Str { value, .. })),
                    ..
                })) => {
                    if value == "use server" {
                        self.export_info.directives.insert("server".to_string());
                    }
                    if value == "use client" {
                        self.export_info.directives.insert("client".to_string());
                    }
                }
                _ => {}
            }
        }
    }

    fn visit_export_decl(&mut self, export_decl: &ExportDecl) {
        match &export_decl.decl {
            Decl::Var(box var_decl) => {
                for decl in &var_decl.decls {
                    if let Pat::Ident(id) = &decl.name {
                        if id.sym == "runtime" {
                            self.export_info.runtime = decl
                                .init
                                .as_ref()
                                .map(|init| {
                                    if let Expr::Lit(Lit::Str(Str { value, .. })) = &**init {
                                        Some(value.to_string())
                                    } else {
                                        None
                                    }
                                })
                                .flatten();
                        } else if id.sym == "preferredRegion" {
                            if let Some(init) = &decl.init {
                                if let Expr::Array(arr) = &**init {
                                    for expr in &arr.elems {
                                        if let Some(ExprOrSpread {
                                            expr: box Expr::Lit(Lit::Str(Str { value, .. })),
                                            ..
                                        }) = expr
                                        {
                                            self.export_info
                                                .preferred_region
                                                .push(value.to_string());
                                        }
                                    }
                                } else if let Expr::Lit(Lit::Str(Str { value, .. })) = &**init {
                                    self.export_info.preferred_region.push(value.to_string());
                                }
                            }
                        } else {
                            self.export_info.extra_properties.insert(id.sym.to_string());
                        }
                    }
                }

                /*
                        if (
                  node.type === 'ExportDeclaration' &&
                  node.declaration?.type === 'VariableDeclaration'
                ) {
                  const id = node.declaration?.declarations[0]?.id.value
                  if (exportsSet.has(id)) {
                    ssg = id === 'getStaticProps'
                    ssr = id === 'getServerSideProps'
                    generateImageMetadata = id === 'generateImageMetadata'
                    generateSitemaps = id === 'generateSitemaps'
                    generateStaticParams = id === 'generateStaticParams'
                  }
                }
                         */
            }
            Decl::Fn(fn_decl) => {
                /*
                      if (
                  node.type === 'ExportDeclaration' &&
                  node.declaration?.type === 'FunctionDeclaration' &&
                  exportsSet.has(node.declaration.identifier?.value)
                ) {
                  const id = node.declaration.identifier.value
                  ssg = id === 'getStaticProps'
                  ssr = id === 'getServerSideProps'
                  generateImageMetadata = id === 'generateImageMetadata'
                  generateSitemaps = id === 'generateSitemaps'
                  generateStaticParams = id === 'generateStaticParams'
                }
                       */
            }
            _ => {}
        }
    }

    fn visit_named_export(&mut self, named_export: &NamedExport) {
      /*
        if (node.type === 'ExportNamedDeclaration') {
          const values = node.specifiers.map(
            (specifier: any) =>
              specifier.type === 'ExportSpecifier' &&
              specifier.orig?.type === 'Identifier' &&
              specifier.orig?.value
          )

          for (const value of values) {
            if (!ssg && value === 'getStaticProps') ssg = true
            if (!ssr && value === 'getServerSideProps') ssr = true
            if (!generateImageMetadata && value === 'generateImageMetadata')
              generateImageMetadata = true
            if (!generateSitemaps && value === 'generateSitemaps')
              generateSitemaps = true
            if (!generateStaticParams && value === 'generateStaticParams')
              generateStaticParams = true
            if (!runtime && value === 'runtime')
              warnInvalidValue(
                pageFilePath,
                'runtime',
                'it was not assigned to a string literal'
              )
            if (!preferredRegion && value === 'preferredRegion')
              warnInvalidValue(
                pageFilePath,
                'preferredRegion',
                'it was not assigned to a string literal or an array of string literals'
              )
          }
        }
*/

    }
}
