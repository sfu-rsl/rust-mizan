from .rustfmt import FormatCompactMutation, FormatExpandedMutation
from .insertions import (
    BenignCommentsMutation,
    BenignBlocksMutation,
    MalignantCommentsMutation,
    MalignantBlocksMutation,
)
from .mizan_mut import (
    ForToWhileMutation,
    WhileToLoopMutation,
    IfElseReorderMutation,
    DeriveReorderMutation,
    TraitBoundReorderMutation,
    UseReorderMutation,
    ArithmeticIdentityMutation,
    MizanMutAllMutation,
)
from .rename import (
    BenignRenameFnMutation,
    BenignRenameVarMutation,
    MalignantRenameFnMutation,
    MalignantRenameVarMutation,
)
from .comment_removal import CommentRemovalMutation

MUTATION_REGISTRY = {
    "format-compact": FormatCompactMutation,
    "format-expanded": FormatExpandedMutation,
    "benign-comments": BenignCommentsMutation,
    "benign-blocks": BenignBlocksMutation,
    "malignant-comments": MalignantCommentsMutation,
    "malignant-blocks": MalignantBlocksMutation,
    "mizan-mut-for-to-while": ForToWhileMutation,
    "mizan-mut-while-to-loop": WhileToLoopMutation,
    "mizan-mut-if-else-reorder": IfElseReorderMutation,
    "mizan-mut-derive-reorder": DeriveReorderMutation,
    "mizan-mut-trait-bound-reorder": TraitBoundReorderMutation,
    "mizan-mut-use-reorder": UseReorderMutation,
    "mizan-mut-arithmetic-identity": ArithmeticIdentityMutation,
    "mizan-mut-all": MizanMutAllMutation,
    "benign-rename-fn": BenignRenameFnMutation,
    "benign-rename-var": BenignRenameVarMutation,
    "malignant-rename-fn": MalignantRenameFnMutation,
    "malignant-rename-var": MalignantRenameVarMutation,
    "remove-comments": CommentRemovalMutation,
}

__all__ = [
    "FormatCompactMutation",
    "FormatExpandedMutation",
    "BenignCommentsMutation",
    "BenignBlocksMutation",
    "MalignantCommentsMutation",
    "MalignantBlocksMutation",
    "ForToWhileMutation",
    "WhileToLoopMutation",
    "IfElseReorderMutation",
    "DeriveReorderMutation",
    "TraitBoundReorderMutation",
    "UseReorderMutation",
    "ArithmeticIdentityMutation",
    "MizanMutAllMutation",
    "BenignRenameFnMutation",
    "BenignRenameVarMutation",
    "MalignantRenameFnMutation",
    "MalignantRenameVarMutation",
    "CommentRemovalMutation",
    "MUTATION_REGISTRY",
]
