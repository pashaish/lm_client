#!/bin/bash

# Check if enough arguments are provided
if [ "$#" -lt 2 ]; then
    echo "Usage: $0 <parent_component> <component_name> [nested_components...]"
    echo "Example: $0 app sidebar"
    exit 1
fi

# Extract the parent component and component name
PARENT_COMPONENT=$1
COMPONENT_NAME=$2

# Build the path based on arguments
COMPONENT_PATH="src/app/$PARENT_COMPONENT"

# If there are additional arguments, they're treated as nested components
if [ "$#" -gt 2 ]; then
    shift 2  # Remove the first two arguments
    for NESTED in "$@"; do
        COMPONENT_PATH="$COMPONENT_PATH/$NESTED"
    done
fi

COMPONENT_PATH="$COMPONENT_PATH/$COMPONENT_NAME"

# Create directory if it doesn't exist
mkdir -p "$COMPONENT_PATH"

# Function to capitalize first letter
capitalize() {
    echo "$(tr '[:lower:]' '[:upper:]' <<< ${1:0:1})${1:1}"
}

COMPONENT_CAPITALIZE=$(capitalize "$COMPONENT_NAME")

# Create state file
cat > "$COMPONENT_PATH/${COMPONENT_NAME}_state.rs" << EOL

#[derive(Debug, Clone)]
pub enum Message {
}

#[derive(Debug, Clone)]
pub struct ${COMPONENT_CAPITALIZE} {
}

impl ${COMPONENT_CAPITALIZE} {
    pub fn new() -> (Self, iced::Task<Message>) {
        let mut tasks = vec![]; 

        (
            Self {},
            iced::Task::batch(tasks)
        )
    }
}
EOL

# Create view file
cat > "$COMPONENT_PATH/${COMPONENT_NAME}_view.rs" << EOL
use iced::Element;
use framework::Context;

use super::${COMPONENT_CAPITALIZE};

impl ${COMPONENT_CAPITALIZE} {
    pub fn view(&self, ctx: &Context) -> Element<super::Message> {
        iced::widget::text("${COMPONENT_CAPITALIZE}").into()
    }
}
EOL

# Create update file
cat > "$COMPONENT_PATH/${COMPONENT_NAME}_update.rs" << EOL
use super::${COMPONENT_CAPITALIZE};
use iced::Task;
use framework::Context;

impl ${COMPONENT_CAPITALIZE} {
    pub fn update(&mut self, ctx: &mut Context, message: super::Message) -> Task<super::Message> {
        match message {
        }
    }
}
EOL

# Create subscription file
cat > "$COMPONENT_PATH/${COMPONENT_NAME}_subscription.rs" << EOL
use iced::Subscription;
use framework::Context;

use super::${COMPONENT_CAPITALIZE};

impl ${COMPONENT_CAPITALIZE} {
    pub fn subscription(&self, ctx: &Context) -> Subscription<super::Message> {
        let mut subs = vec![];

        Subscription::batch(subs)
    }
}
EOL

# Create mod.rs
cat > "$COMPONENT_PATH/mod.rs" << EOL
mod ${COMPONENT_NAME}_view;
mod ${COMPONENT_NAME}_update;
mod ${COMPONENT_NAME}_subscription;
mod ${COMPONENT_NAME}_state;

pub use ${COMPONENT_NAME}_state::${COMPONENT_CAPITALIZE};
pub use ${COMPONENT_NAME}_state::Message;
EOL

echo "Component ${COMPONENT_CAPITALIZE} created at ${COMPONENT_PATH}"
echo "Created files:"
echo "- ${COMPONENT_NAME}_state.rs"
echo "- ${COMPONENT_NAME}_view.rs"
echo "- ${COMPONENT_NAME}_update.rs"
echo "- ${COMPONENT_NAME}_subscription.rs"
echo "- mod.rs"

# Make the script executable
chmod +x "$0"
