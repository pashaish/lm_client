Improvements:
 - [ ] Abort Chat Completion (currently we cancel receiving, but the request itself continues in the BACKGROUND)

Global Improvements:
 - [ ] Implement a notification system to send messages to the user about errors and tips, such as (select the required model)
 - [ ] Add a custom error type to determine whether to show a notification and what type
 - [ ] Return `Result` for all errors instead of panicking
   And catch these errors at the root to send them as notifications
 - [ ] Custom markdown parser/render with text selection support

Distant Global Improvements:
 - [ ] Favorite models from provider settings, display only them in selectors, possibly add labels (e.g., embedding model, etc.)
 - [ ] Learn to convert PDFs to text and then into RAG
