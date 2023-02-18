# An experiment to better understand shaders in bevy.

## Expected behavior

- The pulse shader is intended to apply after the green color shader.
- The `pulse` shader should maintain the green color and animate the alpha between 0 and 1.

## Actual behaviour

Sometimes when this app is launched the rectangle is shown in just green:

<img width="501" alt="Screenshot 2023-02-18 at 11 00 20" src="https://user-images.githubusercontent.com/645812/219856796-9e67e42b-4394-41e5-bd60-74712ec256a8.png">

Somtimes when the app is launched it animates between green and blue:

https://user-images.githubusercontent.com/645812/219856794-224ca456-f9f0-4a2e-8626-51aad433f9e0.mov
