## Project Plan for Levi: Rust Web App Bootstrapper

### Step 1: Define Levi’s Features and Templates

- **Duration**: 1 days
- **Task**:
  - List all the combinations of Rust frameworks and ORMs you want to support (e.g., Axum with Diesel, Actix with Sea-ORM).
- **Output**:
  - A document detailing each template’s structure and the features Levi will offer, such as scaffolding, optional Docker integration, and README generation.

### Step 2: Develop the CLI Framework for Levi

- **Duration**: 1 weeks
- **Task**:
  - Utilize `clap` to create the basic CLI structure that allows users to choose options like framework and ORM.
- **Output**:
  - A working CLI that can parse user input and select the correct template based on arguments.

### Step 3: Create and Organize Templates for Levi

- **Duration**: 2 weeks
- **Task**:
  - Create a base project in the `templates` directory for each combination decided in Step 1.
  - Ensure each template is a minimal but functional starter kit for the specified stack.
- **Output**:
  - A repository of organized templates that can be cloned or copied.ё

### Step 4: Implement Template Processing Logic in Levi

- **Duration**: 1 weeks
- **Task**:
  - Write the logic to copy the selected template from the repository to the user's desired directory.
  - Include post-processing to customize template files (e.g., setting project names).
- **Output**:
  - Functional part of Levi that sets up a new project based on user input.

### Step 5: Testing and Documentation for Levi

- **Duration**: 1 week
- **Task**:
  - Test each template and CLI commands to ensure they work as expected across different environments.
  - Write clear documentation on how to install and use Levi, including examples.
- **Output**:
  - A well-documented and tested version of Levi ready for user feedback.

### Step 6: Release and Iterate Levi

- **Duration**: Ongoing
- **Task**:
  - Package your CLI tool and release it on GitHub or crates.io.
  - Collect user feedback and refine Levi based on suggestions and reported issues.
- **Output**:
  - Updates and improvements to Levi, ensuring it stays relevant and useful.
