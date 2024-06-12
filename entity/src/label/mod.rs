mod label_meta;
mod user_label;

pub use label_meta::{
    ActiveModel as LabelMetaActiveModel, Column as LabelMetaColumn, Entity as LabelMetaEntity,
    Model as LabelMetaModel,
};

pub use user_label::{
    ActiveModel as UserLabelActiveModel, Column as UserLabelColumn, Entity as UserLabelEntity,
    Model as UserLabelModel,
};
