//! # Runtime library bindings
//!
//! This metadata ensures that the bindings to the runtime functions exist in the current
//! compilation context.

use crate::{
    error::{Error, Result},
    libfuncs::LibfuncHelper,
    utils::BlockExt,
};
use melior::{
    dialect::{func, llvm},
    ir::{
        attribute::{FlatSymbolRefAttribute, StringAttribute, TypeAttribute},
        r#type::{FunctionType, IntegerType},
        Attribute, Block, Identifier, Location, Module, OperationRef, Region, Value,
    },
    Context,
};
use std::{alloc::Layout, collections::HashSet, ffi::c_int, marker::PhantomData};

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
enum RuntimeBinding {
    Pedersen,
    HadesPermutation,
    EcStateTryFinalizeNz,
    EcStateAddMul,
    EcStateInit,
    EcStateAdd,
    EcPointTryNewNz,
    EcPointFromXNz,
    DictNew,
    DictGet,
    DictGasRefund,
    DictDrop,
    DictDup,
    GetGasBuiltin,
    DebugPrint,
    #[cfg(feature = "with-cheatcode")]
    VtableCheatcode,
}

/// Runtime library bindings metadata.
#[derive(Debug)]
pub struct RuntimeBindingsMeta {
    active_map: HashSet<RuntimeBinding>,
    phantom: PhantomData<()>,
}

impl RuntimeBindingsMeta {
    /// Register if necessary, then invoke the `debug::print()` function.
    #[allow(clippy::too_many_arguments)]
    pub fn libfunc_debug_print<'c, 'a>(
        &mut self,
        context: &'c Context,
        module: &Module,
        block: &'a Block<'c>,
        target_fd: Value<'c, '_>,
        values_ptr: Value<'c, '_>,
        values_len: Value<'c, '_>,
        location: Location<'c>,
    ) -> Result<Value<'c, 'a>>
    where
        'c: 'a,
    {
        if self.active_map.insert(RuntimeBinding::DebugPrint) {
            module.body().append_operation(func::func(
                context,
                StringAttribute::new(context, "cairo_native__libfunc__debug__print"),
                TypeAttribute::new(
                    FunctionType::new(
                        context,
                        &[
                            IntegerType::new(context, 32).into(),
                            llvm::r#type::pointer(context, 0),
                            IntegerType::new(context, 32).into(),
                        ],
                        &[IntegerType::new(context, 32).into()],
                    )
                    .into(),
                ),
                Region::new(),
                &[
                    (
                        Identifier::new(context, "sym_visibility"),
                        StringAttribute::new(context, "private").into(),
                    ),
                    (
                        Identifier::new(context, "llvm.linkage"),
                        Attribute::parse(context, "#llvm.linkage<external>")
                            .ok_or(Error::ParseAttributeError)?,
                    ),
                ],
                Location::unknown(context),
            ));
        }

        Ok(block
            .append_operation(func::call(
                context,
                FlatSymbolRefAttribute::new(context, "cairo_native__libfunc__debug__print"),
                &[target_fd, values_ptr, values_len],
                &[IntegerType::new(context, 32).into()],
                location,
            ))
            .result(0)?
            .into())
    }

    /// Register if necessary, then invoke the `pedersen()` function.
    #[allow(clippy::too_many_arguments)]
    pub fn libfunc_pedersen<'c, 'a>(
        &mut self,
        context: &'c Context,
        module: &Module,
        block: &'a Block<'c>,
        dst_ptr: Value<'c, '_>,
        lhs_ptr: Value<'c, '_>,
        rhs_ptr: Value<'c, '_>,
        location: Location<'c>,
    ) -> Result<OperationRef<'c, 'a>>
    where
        'c: 'a,
    {
        if self.active_map.insert(RuntimeBinding::Pedersen) {
            module.body().append_operation(func::func(
                context,
                StringAttribute::new(context, "cairo_native__libfunc__pedersen"),
                TypeAttribute::new(
                    FunctionType::new(
                        context,
                        &[
                            llvm::r#type::pointer(context, 0),
                            llvm::r#type::pointer(context, 0),
                            llvm::r#type::pointer(context, 0),
                        ],
                        &[],
                    )
                    .into(),
                ),
                Region::new(),
                &[
                    (
                        Identifier::new(context, "sym_visibility"),
                        StringAttribute::new(context, "private").into(),
                    ),
                    (
                        Identifier::new(context, "llvm.linkage"),
                        Attribute::parse(context, "#llvm.linkage<external>")
                            .ok_or(Error::ParseAttributeError)?,
                    ),
                ],
                Location::unknown(context),
            ));
        }

        Ok(block.append_operation(func::call(
            context,
            FlatSymbolRefAttribute::new(context, "cairo_native__libfunc__pedersen"),
            &[dst_ptr, lhs_ptr, rhs_ptr],
            &[],
            location,
        )))
    }

    /// Register if necessary, then invoke the `poseidon()` function.
    /// The passed pointers serve both as in/out pointers. I.E results are stored in the given pointers.
    #[allow(clippy::too_many_arguments)]
    pub fn libfunc_hades_permutation<'c, 'a>(
        &mut self,
        context: &'c Context,
        module: &Module,
        block: &'a Block<'c>,
        op0_ptr: Value<'c, '_>,
        op1_ptr: Value<'c, '_>,
        op2_ptr: Value<'c, '_>,
        location: Location<'c>,
    ) -> Result<OperationRef<'c, 'a>>
    where
        'c: 'a,
    {
        if self.active_map.insert(RuntimeBinding::HadesPermutation) {
            module.body().append_operation(func::func(
                context,
                StringAttribute::new(context, "cairo_native__libfunc__hades_permutation"),
                TypeAttribute::new(
                    FunctionType::new(
                        context,
                        &[
                            llvm::r#type::pointer(context, 0),
                            llvm::r#type::pointer(context, 0),
                            llvm::r#type::pointer(context, 0),
                        ],
                        &[],
                    )
                    .into(),
                ),
                Region::new(),
                &[
                    (
                        Identifier::new(context, "sym_visibility"),
                        StringAttribute::new(context, "private").into(),
                    ),
                    (
                        Identifier::new(context, "llvm.linkage"),
                        Attribute::parse(context, "#llvm.linkage<external>")
                            .ok_or(Error::ParseAttributeError)?,
                    ),
                ],
                Location::unknown(context),
            ));
        }

        Ok(block.append_operation(func::call(
            context,
            FlatSymbolRefAttribute::new(context, "cairo_native__libfunc__hades_permutation"),
            &[op0_ptr, op1_ptr, op2_ptr],
            &[],
            location,
        )))
    }

    /// Register if necessary, then invoke the `ec_point_from_x_nz()` function.
    pub fn libfunc_ec_point_from_x_nz<'c, 'a>(
        &mut self,
        context: &'c Context,
        module: &Module,
        block: &'a Block<'c>,
        point_ptr: Value<'c, '_>,
        location: Location<'c>,
    ) -> Result<OperationRef<'c, 'a>>
    where
        'c: 'a,
    {
        if self.active_map.insert(RuntimeBinding::EcPointFromXNz) {
            module.body().append_operation(func::func(
                context,
                StringAttribute::new(context, "cairo_native__libfunc__ec__ec_point_from_x_nz"),
                TypeAttribute::new(
                    FunctionType::new(
                        context,
                        &[llvm::r#type::pointer(context, 0)],
                        &[IntegerType::new(context, 1).into()],
                    )
                    .into(),
                ),
                Region::new(),
                &[
                    (
                        Identifier::new(context, "sym_visibility"),
                        StringAttribute::new(context, "private").into(),
                    ),
                    (
                        Identifier::new(context, "llvm.linkage"),
                        Attribute::parse(context, "#llvm.linkage<external>")
                            .ok_or(Error::ParseAttributeError)?,
                    ),
                ],
                Location::unknown(context),
            ));
        }

        Ok(block.append_operation(func::call(
            context,
            FlatSymbolRefAttribute::new(context, "cairo_native__libfunc__ec__ec_point_from_x_nz"),
            &[point_ptr],
            &[IntegerType::new(context, 1).into()],
            location,
        )))
    }

    /// Register if necessary, then invoke the `ec_point_try_new_nz()` function.
    pub fn libfunc_ec_point_try_new_nz<'c, 'a>(
        &mut self,
        context: &'c Context,
        module: &Module,
        block: &'a Block<'c>,
        point_ptr: Value<'c, '_>,
        location: Location<'c>,
    ) -> Result<OperationRef<'c, 'a>>
    where
        'c: 'a,
    {
        if self.active_map.insert(RuntimeBinding::EcPointTryNewNz) {
            module.body().append_operation(func::func(
                context,
                StringAttribute::new(context, "cairo_native__libfunc__ec__ec_point_try_new_nz"),
                TypeAttribute::new(
                    FunctionType::new(
                        context,
                        &[llvm::r#type::pointer(context, 0)],
                        &[IntegerType::new(context, 1).into()],
                    )
                    .into(),
                ),
                Region::new(),
                &[
                    (
                        Identifier::new(context, "sym_visibility"),
                        StringAttribute::new(context, "private").into(),
                    ),
                    (
                        Identifier::new(context, "llvm.linkage"),
                        Attribute::parse(context, "#llvm.linkage<external>")
                            .ok_or(Error::ParseAttributeError)?,
                    ),
                ],
                Location::unknown(context),
            ));
        }

        Ok(block.append_operation(func::call(
            context,
            FlatSymbolRefAttribute::new(context, "cairo_native__libfunc__ec__ec_point_try_new_nz"),
            &[point_ptr],
            &[IntegerType::new(context, 1).into()],
            location,
        )))
    }

    /// Register if necessary, then invoke the `ec_state_init()` function.
    pub fn libfunc_ec_state_init<'c, 'a>(
        &mut self,
        context: &'c Context,
        module: &Module,
        block: &'a Block<'c>,
        state_ptr: Value<'c, '_>,
        location: Location<'c>,
    ) -> Result<OperationRef<'c, 'a>>
    where
        'c: 'a,
    {
        if self.active_map.insert(RuntimeBinding::EcStateInit) {
            module.body().append_operation(func::func(
                context,
                StringAttribute::new(context, "cairo_native__libfunc__ec__ec_state_init"),
                TypeAttribute::new(
                    FunctionType::new(context, &[llvm::r#type::pointer(context, 0)], &[]).into(),
                ),
                Region::new(),
                &[
                    (
                        Identifier::new(context, "sym_visibility"),
                        StringAttribute::new(context, "private").into(),
                    ),
                    (
                        Identifier::new(context, "llvm.linkage"),
                        Attribute::parse(context, "#llvm.linkage<external>")
                            .ok_or(Error::ParseAttributeError)?,
                    ),
                ],
                Location::unknown(context),
            ));
        }

        Ok(block.append_operation(func::call(
            context,
            FlatSymbolRefAttribute::new(context, "cairo_native__libfunc__ec__ec_state_init"),
            &[state_ptr],
            &[],
            location,
        )))
    }

    /// Register if necessary, then invoke the `ec_state_add()` function.
    pub fn libfunc_ec_state_add<'c, 'a>(
        &mut self,
        context: &'c Context,
        module: &Module,
        block: &'a Block<'c>,
        state_ptr: Value<'c, '_>,
        point_ptr: Value<'c, '_>,
        location: Location<'c>,
    ) -> Result<OperationRef<'c, 'a>>
    where
        'c: 'a,
    {
        if self.active_map.insert(RuntimeBinding::EcStateAdd) {
            module.body().append_operation(func::func(
                context,
                StringAttribute::new(context, "cairo_native__libfunc__ec__ec_state_add"),
                TypeAttribute::new(
                    FunctionType::new(
                        context,
                        &[
                            llvm::r#type::pointer(context, 0),
                            llvm::r#type::pointer(context, 0),
                        ],
                        &[],
                    )
                    .into(),
                ),
                Region::new(),
                &[
                    (
                        Identifier::new(context, "sym_visibility"),
                        StringAttribute::new(context, "private").into(),
                    ),
                    (
                        Identifier::new(context, "llvm.linkage"),
                        Attribute::parse(context, "#llvm.linkage<external>")
                            .ok_or(Error::ParseAttributeError)?,
                    ),
                ],
                Location::unknown(context),
            ));
        }

        Ok(block.append_operation(func::call(
            context,
            FlatSymbolRefAttribute::new(context, "cairo_native__libfunc__ec__ec_state_add"),
            &[state_ptr, point_ptr],
            &[],
            location,
        )))
    }

    /// Register if necessary, then invoke the `ec_state_add_mul()` function.
    #[allow(clippy::too_many_arguments)]
    pub fn libfunc_ec_state_add_mul<'c, 'a>(
        &mut self,
        context: &'c Context,
        module: &Module,
        block: &'a Block<'c>,
        state_ptr: Value<'c, '_>,
        scalar_ptr: Value<'c, '_>,
        point_ptr: Value<'c, '_>,
        location: Location<'c>,
    ) -> Result<OperationRef<'c, 'a>>
    where
        'c: 'a,
    {
        if self.active_map.insert(RuntimeBinding::EcStateAddMul) {
            module.body().append_operation(func::func(
                context,
                StringAttribute::new(context, "cairo_native__libfunc__ec__ec_state_add_mul"),
                TypeAttribute::new(
                    FunctionType::new(
                        context,
                        &[
                            llvm::r#type::pointer(context, 0),
                            llvm::r#type::pointer(context, 0),
                            llvm::r#type::pointer(context, 0),
                        ],
                        &[],
                    )
                    .into(),
                ),
                Region::new(),
                &[
                    (
                        Identifier::new(context, "sym_visibility"),
                        StringAttribute::new(context, "private").into(),
                    ),
                    (
                        Identifier::new(context, "llvm.linkage"),
                        Attribute::parse(context, "#llvm.linkage<external>")
                            .ok_or(Error::ParseAttributeError)?,
                    ),
                ],
                Location::unknown(context),
            ));
        }

        Ok(block.append_operation(func::call(
            context,
            FlatSymbolRefAttribute::new(context, "cairo_native__libfunc__ec__ec_state_add_mul"),
            &[state_ptr, scalar_ptr, point_ptr],
            &[],
            location,
        )))
    }

    pub fn libfunc_ec_state_try_finalize_nz<'c, 'a>(
        &mut self,
        context: &'c Context,
        module: &Module,
        block: &'a Block<'c>,
        point_ptr: Value<'c, '_>,
        state_ptr: Value<'c, '_>,
        location: Location<'c>,
    ) -> Result<OperationRef<'c, 'a>>
    where
        'c: 'a,
    {
        if self.active_map.insert(RuntimeBinding::EcStateTryFinalizeNz) {
            module.body().append_operation(func::func(
                context,
                StringAttribute::new(
                    context,
                    "cairo_native__libfunc__ec__ec_state_try_finalize_nz",
                ),
                TypeAttribute::new(
                    FunctionType::new(
                        context,
                        &[
                            llvm::r#type::pointer(context, 0),
                            llvm::r#type::pointer(context, 0),
                        ],
                        &[IntegerType::new(context, 1).into()],
                    )
                    .into(),
                ),
                Region::new(),
                &[
                    (
                        Identifier::new(context, "sym_visibility"),
                        StringAttribute::new(context, "private").into(),
                    ),
                    (
                        Identifier::new(context, "llvm.linkage"),
                        Attribute::parse(context, "#llvm.linkage<external>")
                            .ok_or(Error::ParseAttributeError)?,
                    ),
                ],
                location,
            ));
        }

        Ok(block.append_operation(func::call(
            context,
            FlatSymbolRefAttribute::new(
                context,
                "cairo_native__libfunc__ec__ec_state_try_finalize_nz",
            ),
            &[point_ptr, state_ptr],
            &[IntegerType::new(context, 1).into()],
            location,
        )))
    }

    /// Register if necessary, then invoke the `dict_alloc_new()` function.
    ///
    /// Returns a opaque pointer as the result.
    #[allow(clippy::too_many_arguments)]
    pub fn dict_new<'c, 'a>(
        &mut self,
        context: &'c Context,
        module: &Module,
        block: &'a Block<'c>,
        location: Location<'c>,
        layout: Layout,
    ) -> Result<Value<'c, 'a>>
    where
        'c: 'a,
    {
        let i64_ty = IntegerType::new(context, 64).into();

        if self.active_map.insert(RuntimeBinding::DictNew) {
            module.body().append_operation(func::func(
                context,
                StringAttribute::new(context, "cairo_native__dict_new"),
                TypeAttribute::new(
                    FunctionType::new(
                        context,
                        &[i64_ty, i64_ty],
                        &[llvm::r#type::pointer(context, 0)],
                    )
                    .into(),
                ),
                Region::new(),
                &[
                    (
                        Identifier::new(context, "sym_visibility"),
                        StringAttribute::new(context, "private").into(),
                    ),
                    (
                        Identifier::new(context, "llvm.linkage"),
                        Attribute::parse(context, "#llvm.linkage<external>")
                            .ok_or(Error::ParseAttributeError)?,
                    ),
                ],
                Location::unknown(context),
            ));
        }

        let size = block.const_int_from_type(context, location, layout.size(), i64_ty)?;
        let align = block.const_int_from_type(context, location, layout.align(), i64_ty)?;

        block.append_op_result(func::call(
            context,
            FlatSymbolRefAttribute::new(context, "cairo_native__dict_new"),
            &[size, align],
            &[llvm::r#type::pointer(context, 0)],
            location,
        ))
    }

    /// Register if necessary, then invoke the `dict_alloc_new()` function.
    ///
    /// Returns a opaque pointer as the result.
    #[allow(clippy::too_many_arguments)]
    pub fn dict_drop<'c, 'a>(
        &mut self,
        context: &'c Context,
        module: &Module,
        block: &'a Block<'c>,
        ptr: Value<'c, 'a>,
        drop_fn: Option<Value<'c, 'a>>,
        location: Location<'c>,
    ) -> Result<OperationRef<'c, 'a>>
    where
        'c: 'a,
    {
        if self.active_map.insert(RuntimeBinding::DictDrop) {
            module.body().append_operation(func::func(
                context,
                StringAttribute::new(context, "cairo_native__dict_drop"),
                TypeAttribute::new(
                    FunctionType::new(
                        context,
                        &[
                            llvm::r#type::pointer(context, 0),
                            llvm::r#type::pointer(context, 0),
                        ],
                        &[],
                    )
                    .into(),
                ),
                Region::new(),
                &[
                    (
                        Identifier::new(context, "sym_visibility"),
                        StringAttribute::new(context, "private").into(),
                    ),
                    (
                        Identifier::new(context, "llvm.linkage"),
                        Attribute::parse(context, "#llvm.linkage<external>")
                            .ok_or(Error::ParseAttributeError)?,
                    ),
                ],
                Location::unknown(context),
            ));
        }

        let drop_fn = match drop_fn {
            Some(x) => x,
            None => {
                block.append_op_result(llvm::zero(llvm::r#type::pointer(context, 0), location))?
            }
        };

        Ok(block.append_operation(func::call(
            context,
            FlatSymbolRefAttribute::new(context, "cairo_native__dict_drop"),
            &[ptr, drop_fn],
            &[],
            location,
        )))
    }

    /// Register if necessary, then invoke the `dict_alloc_new()` function.
    ///
    /// Returns a opaque pointer as the result.
    #[allow(clippy::too_many_arguments)]
    pub fn dict_dup<'c, 'a>(
        &mut self,
        context: &'c Context,
        module: &Module,
        block: &'a Block<'c>,
        ptr: Value<'c, 'a>,
        dup_fn: Option<Value<'c, 'a>>,
        location: Location<'c>,
    ) -> Result<Value<'c, 'a>>
    where
        'c: 'a,
    {
        if self.active_map.insert(RuntimeBinding::DictDup) {
            module.body().append_operation(func::func(
                context,
                StringAttribute::new(context, "cairo_native__dict_dup"),
                TypeAttribute::new(
                    FunctionType::new(
                        context,
                        &[
                            llvm::r#type::pointer(context, 0),
                            llvm::r#type::pointer(context, 0),
                        ],
                        &[llvm::r#type::pointer(context, 0)],
                    )
                    .into(),
                ),
                Region::new(),
                &[
                    (
                        Identifier::new(context, "sym_visibility"),
                        StringAttribute::new(context, "private").into(),
                    ),
                    (
                        Identifier::new(context, "llvm.linkage"),
                        Attribute::parse(context, "#llvm.linkage<external>")
                            .ok_or(Error::ParseAttributeError)?,
                    ),
                ],
                Location::unknown(context),
            ));
        }

        let dup_fn = match dup_fn {
            Some(x) => x,
            None => {
                block.append_op_result(llvm::zero(llvm::r#type::pointer(context, 0), location))?
            }
        };

        block.append_op_result(func::call(
            context,
            FlatSymbolRefAttribute::new(context, "cairo_native__dict_dup"),
            &[ptr, dup_fn],
            &[llvm::r#type::pointer(context, 0)],
            location,
        ))
    }

    /// Register if necessary, then invoke the `dict_get()` function.
    ///
    /// Gets the value for a given key, the returned pointer is null if not found.
    ///
    /// Returns a opaque pointer as the result.
    #[allow(clippy::too_many_arguments)]
    pub fn dict_get<'c, 'a>(
        &mut self,
        context: &'c Context,
        helper: &LibfuncHelper<'c, 'a>,
        block: &'a Block<'c>,
        dict_ptr: Value<'c, 'a>, // ptr to the dict
        key_ptr: Value<'c, 'a>,  // key must be a ptr to Felt
        location: Location<'c>,
    ) -> Result<(Value<'c, 'a>, Value<'c, 'a>)>
    where
        'c: 'a,
    {
        if self.active_map.insert(RuntimeBinding::DictGet) {
            helper.body().append_operation(func::func(
                context,
                StringAttribute::new(context, "cairo_native__dict_get"),
                TypeAttribute::new(
                    FunctionType::new(
                        context,
                        &[
                            llvm::r#type::pointer(context, 0),
                            llvm::r#type::pointer(context, 0),
                            llvm::r#type::pointer(context, 0),
                        ],
                        &[IntegerType::new(context, c_int::BITS).into()],
                    )
                    .into(),
                ),
                Region::new(),
                &[
                    (
                        Identifier::new(context, "sym_visibility"),
                        StringAttribute::new(context, "private").into(),
                    ),
                    (
                        Identifier::new(context, "llvm.linkage"),
                        Attribute::parse(context, "#llvm.linkage<external>")
                            .ok_or(Error::ParseAttributeError)?,
                    ),
                ],
                Location::unknown(context),
            ));
        }

        let value_ptr = helper.init_block().alloca1(
            context,
            location,
            llvm::r#type::pointer(context, 0),
            align_of::<*mut ()>(),
        )?;

        let is_present = block.append_op_result(func::call(
            context,
            FlatSymbolRefAttribute::new(context, "cairo_native__dict_get"),
            &[dict_ptr, key_ptr, value_ptr],
            &[IntegerType::new(context, c_int::BITS).into()],
            location,
        ))?;

        let value_ptr = block.load(
            context,
            location,
            value_ptr,
            llvm::r#type::pointer(context, 0),
        )?;

        Ok((is_present, value_ptr))
    }

    /// Register if necessary, then invoke the `dict_gas_refund()` function.
    ///
    /// Compute the total gas refund for the dictionary.
    ///
    /// Returns a u64 of the result.
    #[allow(clippy::too_many_arguments)]
    pub fn dict_gas_refund<'c, 'a>(
        &mut self,
        context: &'c Context,
        module: &Module,
        block: &'a Block<'c>,
        dict_ptr: Value<'c, 'a>, // ptr to the dict
        location: Location<'c>,
    ) -> Result<OperationRef<'c, 'a>>
    where
        'c: 'a,
    {
        if self.active_map.insert(RuntimeBinding::DictGasRefund) {
            module.body().append_operation(func::func(
                context,
                StringAttribute::new(context, "cairo_native__dict_gas_refund"),
                TypeAttribute::new(
                    FunctionType::new(
                        context,
                        &[llvm::r#type::pointer(context, 0)],
                        &[IntegerType::new(context, 64).into()],
                    )
                    .into(),
                ),
                Region::new(),
                &[
                    (
                        Identifier::new(context, "sym_visibility"),
                        StringAttribute::new(context, "private").into(),
                    ),
                    (
                        Identifier::new(context, "llvm.linkage"),
                        Attribute::parse(context, "#llvm.linkage<external>")
                            .ok_or(Error::ParseAttributeError)?,
                    ),
                ],
                Location::unknown(context),
            ));
        }

        Ok(block.append_operation(func::call(
            context,
            FlatSymbolRefAttribute::new(context, "cairo_native__dict_gas_refund"),
            &[dict_ptr],
            &[IntegerType::new(context, 64).into()],
            location,
        )))
    }

    // Register if necessary, then invoke the `set_gas_builtin()` function.
    #[allow(clippy::too_many_arguments)]
    pub fn get_gas_builtin<'c, 'a>(
        &mut self,
        context: &'c Context,
        module: &Module,
        block: &'a Block<'c>,
        location: Location<'c>,
    ) -> Result<OperationRef<'c, 'a>>
    where
        'c: 'a,
    {
        if self.active_map.insert(RuntimeBinding::GetGasBuiltin) {
            module.body().append_operation(func::func(
                context,
                StringAttribute::new(context, "cairo_native__get_costs_builtin"),
                TypeAttribute::new(
                    FunctionType::new(context, &[], &[llvm::r#type::pointer(context, 0)]).into(),
                ),
                Region::new(),
                &[
                    (
                        Identifier::new(context, "sym_visibility"),
                        StringAttribute::new(context, "private").into(),
                    ),
                    (
                        Identifier::new(context, "llvm.linkage"),
                        Attribute::parse(context, "#llvm.linkage<external>")
                            .ok_or(Error::ParseAttributeError)?,
                    ),
                ],
                Location::unknown(context),
            ));
        }

        Ok(block.append_operation(func::call(
            context,
            FlatSymbolRefAttribute::new(context, "cairo_native__get_costs_builtin"),
            &[],
            &[llvm::r#type::pointer(context, 0)],
            location,
        )))
    }

    /// Register if necessary, then invoke the `vtable_cheatcode()` runtime function.
    ///
    /// Calls the cheatcode syscall with the given arguments.
    ///
    /// The result is stored in `result_ptr`.
    #[allow(clippy::too_many_arguments)]
    #[cfg(feature = "with-cheatcode")]
    pub fn vtable_cheatcode<'c, 'a>(
        &mut self,
        context: &'c Context,
        module: &Module,
        block: &'a Block<'c>,
        location: Location<'c>,
        result_ptr: Value<'c, 'a>,
        selector_ptr: Value<'c, 'a>,
        args: Value<'c, 'a>,
    ) -> Result<OperationRef<'c, 'a>>
    where
        'c: 'a,
    {
        if self.active_map.insert(RuntimeBinding::VtableCheatcode) {
            module.body().append_operation(func::func(
                context,
                StringAttribute::new(context, "cairo_native__vtable_cheatcode"),
                TypeAttribute::new(
                    FunctionType::new(
                        context,
                        &[
                            llvm::r#type::pointer(context, 0),
                            llvm::r#type::pointer(context, 0),
                            llvm::r#type::pointer(context, 0),
                        ],
                        &[],
                    )
                    .into(),
                ),
                Region::new(),
                &[
                    (
                        Identifier::new(context, "sym_visibility"),
                        StringAttribute::new(context, "private").into(),
                    ),
                    (
                        Identifier::new(context, "llvm.linkage"),
                        Attribute::parse(context, "#llvm.linkage<external>")
                            .ok_or(Error::ParseAttributeError)?,
                    ),
                ],
                Location::unknown(context),
            ));
        }

        Ok(block.append_operation(func::call(
            context,
            FlatSymbolRefAttribute::new(context, "cairo_native__vtable_cheatcode"),
            &[result_ptr, selector_ptr, args],
            &[],
            location,
        )))
    }
}

impl Default for RuntimeBindingsMeta {
    fn default() -> Self {
        Self {
            active_map: HashSet::new(),
            phantom: PhantomData,
        }
    }
}
