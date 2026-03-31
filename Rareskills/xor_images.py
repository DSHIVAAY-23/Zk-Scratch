from PIL import Image
import numpy as np

def xor_images(image1_path, image2_path, output_path):
    # Open the images
    img1 = Image.open(image1_path).convert('RGB')
    img2 = Image.open(image2_path).convert('RGB')

    # Ensure both images have the same size
    if img1.size != img2.size:
        raise ValueError("Images must have the same dimensions")

    # Convert images to numpy arrays
    img1_array = np.array(img1)
    img2_array = np.array(img2)

    # Perform XOR operation on the RGB values
    result_array = np.bitwise_xor(img1_array, img2_array)

    # Convert the result back to an image
    result_img = Image.fromarray(result_array.astype('uint8'))

    # Save the resulting image
    result_img.save(output_path)
    print(f"XOR image saved to {output_path}")

# Update the file paths to include the correct directory
image1_path = "/home/user/Downloads/lemur_ed66878c338e662d3473f0d98eedbd0d.png"
image2_path = "/home/user/Downloads/flag_7ae18c704272532658c10b5faad06d74.png"
output_path = "xor_result.png"

# Perform the XOR operation
xor_images(image1_path, image2_path, output_path)