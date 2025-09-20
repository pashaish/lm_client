use iced::{widget::{Column, Container, PickList}, Element};
use framework::{types::dto::ProviderDTO, Context};

use super::ModelPicker;

impl ModelPicker {
    pub fn view(&self, _ctx: &Context) -> Element<super::Message> {
        let mut main_column = Column::new();

        main_column = main_column
            .push(self.view_provider_selector(
                self.providers.clone(),
                self.selected_provider.clone(),
                super::Message::SelectProvider,
            ))
            .push(self.view_model_selector(
                self.models.clone(),
                self.selected_model.clone(),
                super::Message::SelectModel,
            ));

            
        Container::new(main_column)
            .padding(5)
            .width(iced::Length::Fill)
            .into()
    }


    fn view_provider_selector(
        &self,
        providers: Vec<ProviderDTO>,
        selected_provider: Option<ProviderDTO>,
        message: impl Fn(ProviderDTO) -> super::Message + 'static,
    ) -> Element<super::Message> {
        Container::new(
            PickList::new(providers, selected_provider, message).width(iced::Length::Fill),
        )
        .padding(5)
        .width(iced::Length::Fill)
        .into()
    }

    fn view_model_selector(
        &self,
        models: Vec<String>,
        current_model: Option<String>,
        message: impl Fn(String) -> super::Message + 'static,
    ) -> Element<super::Message> {
        Container::new(PickList::new(models, current_model, message).width(iced::Length::Fill))
            .padding(5)
            .width(iced::Length::Fill)
            .into()
    }
}
