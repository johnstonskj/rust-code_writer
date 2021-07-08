/*!
One-line description.

More detailed description, with

# Example

*/

use crate::model::comments::Comment;
use crate::model::functions::FunctionDecl;
use crate::model::identity::{HasName, Identifier, Namespace};
use crate::model::properties::{HasProperties, Property};
use crate::model::structured_types::StructuredType;
use crate::model::values::{HasType, NamedValue, ValueType};
use crate::model::{Builder, HasDocumentation};
use std::fmt::Debug;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
pub enum Visibility {
    Private,
    Local,
    Package,
    Public,
}

pub trait HasVisibility {
    fn visibility(&self) -> &Option<Visibility>;
    fn has_visibility(&self) -> bool {
        self.visibility().is_some()
    }
    fn set_visibility(&mut self, visibility: Visibility) -> &mut Self
    where
        Self: Sized;
    fn unset_visibility(&mut self) -> &mut Self
    where
        Self: Sized;
    fn is_private(&self) -> bool {
        self.visibility()
            .as_ref()
            .map(|v| v == &Visibility::Private)
            .is_some()
    }
    fn make_private(&mut self) -> &mut Self
    where
        Self: Sized,
    {
        self.set_visibility(Visibility::Private)
    }
    fn is_local(&self) -> bool {
        self.visibility()
            .as_ref()
            .map(|v| v == &Visibility::Local)
            .is_some()
    }
    fn make_local(&mut self) -> &mut Self
    where
        Self: Sized,
    {
        self.set_visibility(Visibility::Local)
    }
    fn is_package(&self) -> bool {
        self.visibility()
            .as_ref()
            .map(|v| v == &Visibility::Package)
            .is_some()
    }
    fn make_package(&mut self) -> &mut Self
    where
        Self: Sized,
    {
        self.set_visibility(Visibility::Package)
    }
    fn is_public(&self) -> bool {
        self.visibility()
            .as_ref()
            .map(|v| v == &Visibility::Public)
            .is_some()
    }
    fn make_public(&mut self) -> &mut Self
    where
        Self: Sized,
    {
        self.set_visibility(Visibility::Public)
    }
}

// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct TypeAlias {
    visibility: Option<Visibility>,
    name: Identifier,
    value_type: ValueType,
}

#[derive(Clone, Debug)]
pub struct Import {
    visibility: Option<Visibility>,
    namespace: Namespace,
    items: Vec<ImportItem>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImportItem {
    name: Identifier,
    alias: Option<Identifier>,
}

// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub enum ModuleContent {
    Import(Import),
    Comment(Comment),
    Structure(StructuredType),
    Constant(NamedValue),
    Variable(NamedValue),
    Function(FunctionDecl),
    Alias(TypeAlias),
    Module(Module),
}

#[derive(Clone, Debug)]
pub struct Module {
    properties: Vec<Property>,
    visibility: Option<Visibility>,
    name: Identifier,
    documentation: Option<String>,
    inline: bool,
    content: Vec<ModuleContent>,
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl_has_visibility!(TypeAlias);

impl_has_name!(TypeAlias);

impl_has_type!(TypeAlias);

impl TypeAlias {
    pub fn new(name: Identifier, target_type: ValueType) -> Self {
        Self {
            visibility: None,
            name,
            value_type: target_type,
        }
    }

    pub fn with_visibility(
        visibility: Visibility,
        name: Identifier,
        target_type: ValueType,
    ) -> Self {
        Self {
            visibility: Some(visibility),
            name,
            value_type: target_type,
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl_has_visibility!(Import);

impl Builder for Import {}

impl From<Namespace> for Import {
    fn from(ns: Namespace) -> Self {
        Self::new(ns)
    }
}

impl Import {
    pub fn new(namespace: Namespace) -> Self {
        Self {
            visibility: None,
            namespace,
            items: Default::default(),
        }
    }

    pub fn with_items(namespace: Namespace, items: Vec<ImportItem>) -> Self {
        Self {
            visibility: None,
            namespace,
            items,
        }
    }

    pub fn with_visibility(visibility: Visibility, namespace: Namespace) -> Self {
        Self {
            visibility: Some(visibility),
            namespace,
            items: Default::default(),
        }
    }

    pub fn namespace(&self) -> &Namespace {
        &self.namespace
    }

    pub fn set_namespace(&mut self, namespace: Namespace) -> &mut Self {
        self.namespace = namespace;
        self
    }

    pub fn items(&self) -> &Vec<ImportItem> {
        &self.items
    }

    pub fn set_items(&mut self, items: Vec<ImportItem>) -> &mut Self {
        self.items = items;
        self
    }

    pub fn add_item(&mut self, item: ImportItem) -> &mut Self {
        self.items.push(item);
        self
    }

    pub fn item(&mut self, item_name: Identifier) -> &mut Self {
        self.add_item(ImportItem::new(item_name))
    }

    pub fn item_with_alias(&mut self, item_name: Identifier, alias_name: Identifier) -> &mut Self {
        self.add_item(ImportItem::with_alias(item_name, alias_name))
    }
}

// ------------------------------------------------------------------------------------------------

impl_has_name!(ImportItem);

impl Builder for ImportItem {}

impl From<Identifier> for ImportItem {
    fn from(id: Identifier) -> Self {
        Self::new(id)
    }
}

impl ImportItem {
    pub fn new(name: Identifier) -> Self {
        Self { name, alias: None }
    }

    pub fn with_alias(name: Identifier, alias: Identifier) -> Self {
        Self {
            name,
            alias: Some(alias),
        }
    }

    pub fn alias(&self) -> &Option<Identifier> {
        &self.alias
    }

    pub fn set_alias(&mut self, alias: Identifier) -> &mut Self {
        self.alias = Some(alias);
        self
    }

    pub fn unset_alias(&mut self) -> &mut Self {
        self.alias = None;
        self
    }
}

// ------------------------------------------------------------------------------------------------

impl_has_properties!(Module);

impl_has_visibility!(Module);

impl_has_name!(Module);

impl_has_documentation!(Module);

impl Builder for Module {}

impl Module {
    pub fn new(name: Identifier) -> Self {
        Self {
            properties: Default::default(),
            visibility: None,
            name,
            documentation: None,
            inline: false,
            content: Default::default(),
        }
    }

    pub fn new_inline(name: Identifier) -> Self {
        Self {
            properties: Default::default(),
            visibility: None,
            name,
            documentation: None,
            inline: true,
            content: Default::default(),
        }
    }

    pub fn is_inline(&self) -> bool {
        self.inline
    }

    pub fn set_inline(&mut self, inline: bool) -> &mut Self {
        self.inline = inline;
        self
    }

    pub fn content(&self) -> &Vec<ModuleContent> {
        &self.content
    }

    pub fn set_content(&mut self, content: Vec<ModuleContent>) -> &mut Self {
        self.content = content;
        self
    }

    pub fn add_content_item(&mut self, content: ModuleContent) -> &mut Self {
        self.content.push(content);
        self
    }

    pub fn add_import(&mut self, content: Import) -> &mut Self {
        self.add_content_item(ModuleContent::Import(content));
        self
    }

    pub fn add_comment(&mut self, content: Comment) -> &mut Self {
        self.add_content_item(ModuleContent::Comment(content));
        self
    }

    pub fn add_structure(&mut self, content: StructuredType) -> &mut Self {
        self.add_content_item(ModuleContent::Structure(content));
        self
    }

    pub fn add_constant(&mut self, content: NamedValue) -> &mut Self {
        self.add_content_item(ModuleContent::Constant(content));
        self
    }

    pub fn add_variable(&mut self, content: NamedValue) -> &mut Self {
        self.add_content_item(ModuleContent::Variable(content));
        self
    }

    pub fn add_function(&mut self, content: FunctionDecl) -> &mut Self {
        self.add_content_item(ModuleContent::Function(content));
        self
    }

    pub fn add_alias(&mut self, content: TypeAlias) -> &mut Self {
        self.add_content_item(ModuleContent::Alias(content));
        self
    }

    pub fn add_sub_module(&mut self, content: Module) -> &mut Self {
        self.add_content_item(ModuleContent::Module(content));
        self
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
