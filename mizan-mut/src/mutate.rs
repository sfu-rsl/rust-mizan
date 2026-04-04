use anyhow::Result;
use clap::ValueEnum;
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
use walkdir::WalkDir;

use crate::mutations::{
    arithmetic_identity::ArithmeticIdentityMutator, derive_reorder::DeriveReorderMutator,
    explicit_where::ExplicitWhereMutator, for_to_while::ForToWhileMutator,
    explicit_where_to_type_params::RemoveExplicitWhereMutator,
    impl_trait_to_generic::ImplTraitToGenericMutator,
    if_else_reorder::IfElseReorderMutator, trait_bound_reorder::TraitBoundReorderMutator,
    use_reorder::UseReorderMutator, while_to_loop::WhileToLoopMutator,
    extraneous_unsafe::ExtraneousUnsafeMutator,
    option_wrap::OptionWrapMutator,
    maybe_uninit_wrap::MaybeUninitWrapMutator,
    manually_drop_wrap::ManuallyDropWrapMutator,
    explicit_return::ExplicitReturnMutator,
    unreachable_panic::UnreachblePanicMutator,
};

#[derive(Debug, Clone, PartialEq, ValueEnum)]
pub enum Mutation {
    // Special
    /// Applies all available mutations
    #[value(name = "all")]
    All,

    // Loop transformations
    /// Converts for loops to while loops
    #[value(name = "for-to-while")]
    ForToWhile,
    /// Converts while loops to loop blocks with breaks
    #[value(name = "while-to-loop")]
    WhileToLoop,

    // Control flow transformations
    /// Reorders if-else branches by negating conditions
    #[value(name = "if-else-reorder")]
    IfElseReorder,

    // Syntax reordering
    /// Randomly reorders traits in derive attributes
    #[value(name = "derive-reorder")]
    DeriveReorder,
    /// Randomly reorders trait bounds in where clauses
    #[value(name = "trait-bound-reorder")]
    TraitBoundReorder,
    /// Randomly reorders items in use statements
    #[value(name = "use-reorder")]
    UseReorder,

    // Expression transformations
    /// Adds arithmetic identity operations (x + N - N)
    #[value(name = "arithmetic-identity")]
    ArithmeticIdentity,

    // Toggle explicit where
    /// Adds explicit where to function signature
    #[value(name = "explicit-where")]
    ExplicitWhere,

    /// Move Simple type bounds from explicit where to type params
    #[value(name = "explicit-where-to-type-params")]
    ExplicitWhereToTypeParams,

    /// Adds extraneous `unsafe {...}` blocks around statements inside functions
    #[value(name = "extraneous-unsafe")]
    ExtraneousUnsafe,

    /// Converts impl form Trait bounds into generic parameters
    #[value(name = "impl-trait-to-generic")]
    ImplTraitToGeneric,

    /// Wraps expressions in redundant Some(..).unwrap() calls.
    #[value(name = "option-wrap")]
    OptionWrap,

    /// Wraps known safe values into a MaybeUninit<T>, automatically dererencing them
    #[value(name = "maybeuninit-wrap")]
    MaybeUninitWrap,

    /// Places owned variables into ManuallyDrop structs, and later unwraps them
    #[value(name = "manuallydrop-wrap")]
    ManuallyDropWrap,

    // Expression transformations
    /// Converts implicit return statements to use explicit syntax at the function level
    #[value(name = "explicit-return")]
    ExplicitReturn,

    // Control flow transformations
    /// Adds an unreachble panic!() to function bodies using a match expression
    #[value(name = "unreachable-panic")]
    UnreachablePanic,
}

/// Apply mutations to a Rust crate
pub fn apply_mutations(
    root: &Path,
    mutations: Vec<Mutation>,
    ignore_files: &[PathBuf],
) -> Result<()> {
    if mutations.is_empty() {
        eprintln!("Error: No mutations specified. Use -m <mutation-type>");
        std::process::exit(1);
    }

    // Expand "all" to include all mutations
    let mutations_to_apply = if mutations.contains(&Mutation::All) {
        vec![
            Mutation::WhileToLoop,
            Mutation::ForToWhile,
            Mutation::IfElseReorder,
            Mutation::DeriveReorder,
            Mutation::TraitBoundReorder,
            Mutation::UseReorder,
            Mutation::ArithmeticIdentity,
            Mutation::ExplicitWhere,
            Mutation::ExtraneousUnsafe,
            Mutation::ImplTraitToGeneric,
            Mutation::OptionWrap,
            Mutation::MaybeUninitWrap,
            Mutation::ManuallyDropWrap,
            Mutation::ExplicitReturn,
            Mutation::UnreachablePanic,
        ]
    } else {
        mutations.clone()
    };

    println!("Processing crate at: {}", root.display());
    println!("Applying mutations: {:?}", mutations_to_apply);
    if !ignore_files.is_empty() {
        println!("Ignoring files: {:?}", ignore_files);
    }

    // Convert ignore paths to absolute paths for comparison
    let absolute_ignore_files: Vec<PathBuf> = ignore_files
        .iter()
        .map(|p| {
            if p.is_absolute() {
                p.clone()
            } else {
                root.join(p)
            }
        })
        .collect();

    let mut files_modified = 0;
    let mut total_files = 0;
    let mut files_skipped = 0;

    for entry in WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "rs"))
        .filter(|e| !e.path().to_str().unwrap_or("").contains("target"))
    {
        let path = entry.path();

        // Check if this file should be ignored
        let should_ignore = absolute_ignore_files.iter().any(|ignore_path| {
            path == ignore_path || path.ends_with(ignore_path)
        });

        if should_ignore {
            files_skipped += 1;
            continue;
        }

        total_files += 1;

        let content = fs::read_to_string(path)?;
        let mut modified_content = content.clone();

        // Apply all mutations sequentially
        for mutation in &mutations_to_apply {
            modified_content = match mutation {
                Mutation::All => unreachable!(),
                Mutation::ForToWhile => ForToWhileMutator::mutate(&modified_content)?,
                Mutation::ArithmeticIdentity => {
                    ArithmeticIdentityMutator::mutate(&modified_content)?
                }
                Mutation::DeriveReorder => DeriveReorderMutator::mutate(&modified_content)?,
                Mutation::TraitBoundReorder => TraitBoundReorderMutator::mutate(&modified_content)?,
                Mutation::UseReorder => UseReorderMutator::mutate(&modified_content)?,
                Mutation::WhileToLoop => WhileToLoopMutator::mutate(&modified_content)?,
                Mutation::IfElseReorder => IfElseReorderMutator::mutate(&modified_content)?,
                Mutation::ExplicitWhere => ExplicitWhereMutator::mutate(&modified_content)?,
                Mutation::ExplicitWhereToTypeParams => {
                    RemoveExplicitWhereMutator::mutate(&modified_content)?
                }
                Mutation::ExtraneousUnsafe => ExtraneousUnsafeMutator::mutate(&modified_content)?,
                Mutation::ImplTraitToGeneric => {
                    ImplTraitToGenericMutator::mutate(&modified_content)?
                }
                Mutation::OptionWrap => OptionWrapMutator::mutate(&modified_content)?,
                Mutation::MaybeUninitWrap => MaybeUninitWrapMutator::mutate(&modified_content)?,
                Mutation::ManuallyDropWrap => ManuallyDropWrapMutator::mutate(&modified_content)?,
                Mutation::ExplicitReturn => ExplicitReturnMutator::mutate(&modified_content)?,
                Mutation::UnreachablePanic => UnreachblePanicMutator::mutate(&modified_content)?,
            };
        }

        // Only format and write if content actually changed
        if modified_content != content {
            let formatted_content = format_code(path.to_str().unwrap(), &modified_content)?;
            fs::write(path, &formatted_content)?;
            files_modified += 1;
        }
    }

    println!(
        "\nSummary: {} of {} files modified",
        files_modified, total_files
    );
    if files_skipped > 0 {
        println!("Skipped {} ignored files", files_skipped);
    }
    Ok(())
}

/// Format Rust code using rustfmt
fn format_code(file_name: &str, code: &str) -> Result<String> {
    let mut child = Command::new("rustfmt")
        .arg("--edition=2021")
        .arg("--config")
        .arg("normalize_doc_attributes=true")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    // Write code to rustfmt's stdin
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(code.as_bytes())?;
    }

    let output = child.wait_with_output()?;

    if output.status.success() {
        Ok(String::from_utf8(output.stdout)?)
    } else {
        eprintln!("Warning: rustfmt failed for {}", file_name);
        Ok(code.to_string())
    }
}
