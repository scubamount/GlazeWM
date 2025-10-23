use std::{
  cell::{Ref, RefCell, RefMut},
  collections::VecDeque,
  rc::Rc,
};

use anyhow::bail;
use uuid::Uuid;
use wm_common::{ContainerDto, Rect, RootContainerDto};

use crate::{
  impl_common_getters, impl_container_debug,
  models::{
    Container, DirectionContainer, Monitor, TilingContainer,
    WindowContainer,
  },
  traits::{CommonGetters, PositionGetters},
};

/// Root node of the container tree.
#[derive(Clone)]
pub struct RootContainer(Rc<RefCell<RootContainerInner>>);

struct RootContainerInner {
  id: Uuid,
  parent: Option<Container>,
  children: VecDeque<Container>,
  child_focus_order: VecDeque<Uuid>,
}

impl Default for RootContainer {
  fn default() -> Self {
    let root = RootContainerInner {
      id: Uuid::new_v4(),
      parent: None,
      children: VecDeque::new(),
      child_focus_order: VecDeque::new(),
    };

    Self(Rc::new(RefCell::new(root)))
  }
}

impl RootContainer {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn monitors(&self) -> Vec<Monitor> {
    self
      .children()
      .into_iter()
      .filter_map(|container| container.as_monitor().cloned())
      .collect()
  }

  pub fn to_dto(&self) -> anyhow::Result<ContainerDto> {
    let children = self
      .children()
      .iter()
      .map(CommonGetters::to_dto)
      .try_collect()?;

    Ok(ContainerDto::Root(RootContainerDto {
      id: self.id(),
      parent_id: None,
      children,
      child_focus_order: self.0.borrow().child_focus_order.clone().into(),
    }))
  }
}

impl_container_debug!(RootContainer);
impl_common_getters!(RootContainer);

impl PositionGetters for RootContainer {
  fn to_rect(&self) -> anyhow::Result<Rect> {
    bail!("Root container does not have a position.")
  }
}
