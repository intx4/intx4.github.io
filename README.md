[Website](https://intx4.github.io) 

# HOW TO

## Add a new blog post
1. Write your blog post in a markdown file and place it in `src/assets/posts` (remember that links in the file, such as images, should link to `/assets/...`, for example `/assets/images/question.jpg` which corresponds to the image in `/src/assets/images/question.jpg`). 
2. Add a preview into `src/blog/previews.rs`. **IMPORTANT:** routes follow increasing numbering, so the link of the preview should follow this numbering (e.g. `blog/post/$index`). This will spawn a previw which will link to the `$index-1`-th post in `src/blog/posts.rs`
3. Add the post to `src/blog/posts.rs`