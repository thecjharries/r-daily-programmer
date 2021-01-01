# CONTRIBUTING

This project is about building something cool that works well and meets some need, no matter how niche. Contributions are welcome and encouraged. Honestly if you've made it this far I think that's pretty neat and I hope you find the solution you're looking for, even if it's not here.

## Structuring Contributions

This project uses [a successful Git branching model](https://nvie.com/posts/a-successful-git-branching-model/) by way of [`gitflow-avh`](https://github.com/petervanderdoes/gitflow-avh). While you don't need to use that specific tool (or any at all), you do need to follow that structure to encourage a good discussion about your proposed changes.

## Testing

If the majority of the project has been tested, please include tests for any new code or update tests (when they exist) for old code. If you've got any questions about good testing, feel free to reach out. Remember that coverage is a number that can and will be gamed. Testing is to make sure your code does what you say it will.

## Submitting Changes

Open a PR and go to town. Provided the changes are well-documented and easy to understand (a good PR body and solid commits go a long way here), you won't see much resistance from me. If they're not, I might suggest some changes or ask questions. Please don't feel like your PR is invalid. If you've taken the time to make changes to my work I'm very excited and want to understand it.

## Style

* If you don't have [EditorConfig](https://editorconfig.org/) handling the little details for you, you're doing too much work.
* `goimports` is a necessity, not a nice-to-have.
* Documentation is equally necessary. While you don't need to write a forty-seven-volume operations manual for your code, you do need to provide enough information that the basic components can be used without forcing the users to dive into the source to figure it out.
* Try to match the style of the surrounding code. There's no prescriptive style guide but following common ones, your editor's suggestions via linters, or explicitly running your own linter never hurts.
