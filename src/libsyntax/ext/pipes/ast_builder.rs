// Functions for building ASTs, without having to fuss with spans.
//
// To start with, it will be use dummy spans, but it might someday do
// something smarter.

import ast::{ident, node_id};
import codemap::span;
import ext::base::mk_ctxt;

fn ident(s: str) -> ast::ident {
    @(copy s)
}

fn empty_span() -> span {
    {lo: 0, hi: 0, expn_info: none}
}

fn span<T>(+x: T) -> ast::spanned<T> {
    {node: x,
     span: empty_span()}
}

fn path(id: ident) -> @ast::path {
    @{span: empty_span(),
      global: false,
      idents: ~[id],
      rp: none,
      types: ~[]}
}

impl methods for ident {
    fn +(id: ident) -> @ast::path {
        path(self) + id
    }
}

impl methods for @ast::path {
    fn +(id: ident) -> @ast::path {
        @{idents: vec::append_one(self.idents, id)
          with *self}
    }

    fn add_ty(ty: @ast::ty) -> @ast::path {
        @{types: vec::append_one(self.types, ty)
          with *self}
    }

    fn add_tys(+tys: ~[@ast::ty]) -> @ast::path {
        @{types: vec::append(self.types, tys)
          with *self}
    }
}

impl ast_builder for ext_ctxt {
    fn ty_param(id: ast::ident, +bounds: ~[ast::ty_param_bound])
        -> ast::ty_param
    {
        {ident: id, id: self.next_id(), bounds: @bounds}
    }

    fn arg(name: ident, ty: @ast::ty) -> ast::arg {
        {mode: ast::infer(self.next_id()),
         ty: ty,
         ident: name,
         // TODO: should this be the same as the infer id?
         id: self.next_id()}
    }

    fn arg_mode(name: ident, ty: @ast::ty, mode: ast::rmode) -> ast::arg {
        {mode: ast::expl(mode),
         ty: ty,
         ident: name,
         id: self.next_id()}
    }

    fn expr_block(e: @ast::expr) -> ast::blk {
        let blk = {view_items: ~[],
                   stmts: ~[],
                   expr: some(e),
                   id: self.next_id(),
                   rules: ast::default_blk};

        {node: blk,
         span: empty_span()}
    }

    fn fn_decl(+inputs: ~[ast::arg],
               output: @ast::ty) -> ast::fn_decl {
        {inputs: inputs,
         output: output,
         purity: ast::impure_fn,
         cf: ast::return_val,
         // TODO: we'll probably want a variant that does constrained
         // types.
         constraints: ~[]}
    }

    fn item(name: ident,
            +node: ast::item_) -> @ast::item {
        @{ident: name,
         attrs: ~[],
         id: self.next_id(),
         node: node,
         vis: ast::public,
         span: empty_span()}
    }

    fn item_fn_poly(name: ident,
                    +inputs: ~[ast::arg],
                    output: @ast::ty,
                    +ty_params: ~[ast::ty_param],
                    +body: ast::blk) -> @ast::item {
        self.item(name,
                  ast::item_fn(self.fn_decl(inputs, output),
                               ty_params,
                               body))
    }

    fn item_fn(name: ident,
               +inputs: ~[ast::arg],
               output: @ast::ty,
               +body: ast::blk) -> @ast::item {
        self.item_fn_poly(name, inputs, output, ~[], body)
    }

    fn item_enum_poly(name: ident,
                      +variants: ~[ast::variant],
                      +ty_params: ~[ast::ty_param]) -> @ast::item {
        self.item(name,
                  ast::item_enum(variants,
                                 ty_params,
                                 ast::rp_none))
    }

    fn item_enum(name: ident,
                 +variants: ~[ast::variant]) -> @ast::item {
        self.item_enum_poly(name, variants, ~[])
    }

    fn variant(name: ident,
               +tys: ~[@ast::ty]) -> ast::variant {
        let args = tys.map(|ty| {ty: ty, id: self.next_id()});

        span({name: name,
              attrs: ~[],
              args: args,
              id: self.next_id(),
              disr_expr: none,
              vis: ast::public})
    }

    fn item_mod(name: ident,
                +items: ~[@ast::item]) -> @ast::item {
        self.item(name,
                  ast::item_mod({
                      view_items: ~[],
                      items: items}))
    }

    fn ty_path(path: @ast::path) -> @ast::ty {
        // TODO: make sure the node ids are legal.
        @{id: self.next_id(),
          node: ast::ty_path(path, self.next_id()),
          span: empty_span()}
    }

    fn item_ty_poly(name: ident,
                    ty: @ast::ty,
                    +params: ~[ast::ty_param]) -> @ast::item {
        self.item(name,
                  ast::item_ty(ty, params, ast::rp_none))
    }

    fn item_ty(name: ident,
               ty: @ast::ty) -> @ast::item {
        self.item_ty_poly(name, ty, ~[])
    }

    fn ty_vars(+ty_params: ~[ast::ty_param]) -> ~[@ast::ty] {
        ty_params.map(|p| self.ty_path(path(p.ident)))
    }
}
