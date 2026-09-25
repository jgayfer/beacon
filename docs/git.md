# Using git

Beacon uses git for version control. Version control helps us keep a source of
truth, while also allowing us to collaborate.

Git can be a complex tool, but for working on art, [GitHub Desktop](https://desktop.github.com/download/)
should be sufficient.

You'll need a GitHub acount to proceed. You may want to be added to the repository
on GitHub to make contributing easier.

## Clone the repository

Once GitHub Desktop is setup, the first thing you'll want to do is to "clone"
this repository.

- Select "Clone a Repository from the Internet"
- Select "URL"
- Use `https://github.com/jgayfer/beacon` as the URL, and click "Clone"

## Making changes

Git uses a system of "branches" and "commits". A branch consists of a linear history
of commits, each representing a snapshot of the project.

The `main` branch points to the current working copy of the game. Generally we
don't "push" commits directly to `main`. Instead, we "branch" off of `main`,
make our changes, commit them, and then "merge" that branch back into `main`.

When we request a branch be merged back into `main`, we create a "pull request"
(PR for short). Really these are "merge requests", but GitHub calls them pull
requests, so that's what we'll use.

### Submit a pull request

In GitHub Desktop, you can see your current branch (`main`) at the top.

To create a branch, click on "Current Branch" and select "New Branch". Name it
something related to what you're working on.

> [!IMPORTANT]  
> When you change branches the files on your system will all change as well.
  Uncommited work can be lost if care isn't taken. Be sure to commit first before
  switching branches.

Now, go ahead and make whatever changes you'd like. Perhaps you're adding a new
Aesprite files to the [`assets`](/assets) directory. Or you're modifying an existing
Aesprite file.

Changes show up in GitHub Desktop as you make them. Once you're satisfied with
them, select all the files you'd like to commmit, click the button on the
bottom left. Ideally you'd also type a title and description for the commit
message.

You can create as many commits as you like. Once ready to submit, click "Publish
branch". This will "push" your branch to the repository on GitHub. Next click
"Preview Pull Request". After reviewing your changes, GitHub will open in the
browser, where you can finalize the pull request.

## Reviewing pull requests

On GitHub, you can browse a [list of open pull reqeusts](https://github.com/jgayfer/beacon/pulls)
for the project.

If you'd like to try out the change set before it is merged, you can "checkout"
the branch locally.

In GitHub Desktop, click on "Current Branch", and then "Pull Requests". Here you
can select any open pull request and check it out locally. 
