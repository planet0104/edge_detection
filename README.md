# image edge detection imitate Retina principle 
本算法模仿视网膜中双极细胞和水平细胞的作用，来提取图像的边缘特征。<br/><br/>
<img src="https://github.com/planet0104/edge_detection/blob/master/table.png"/><br/><br/>
上图中，字母代表图片的像素，B代表双极细胞, H代表水平细胞。红色B点代表当前像素点，那么当前像素点的输出等于4个蓝底像素输出值之和除以4:<br/>
<b>pixel(1,1) = Sum(outH+outB+outH+outB)/4</b><br/>
B和H的输出，根据亮度计算,如果像素亮度超过阈值，B输出255，H输出-255，没有超过阈值，二者都输出0。<br/><br/>
算法运行效果如下：<br/><br/>
<img src="http://planet0104.github.io/rust.png"/><img src="http://planet0104.github.io/rust_out.png"/><br/>
<img src="https://github.com/planet0104/edge_detection/blob/master/r.png"/><img src="https://github.com/planet0104/edge_detection/blob/master/0_out.png"/><br/>
<img src="https://github.com/planet0104/edge_detection/blob/master/0.png"/><img src="https://github.com/planet0104/edge_detection/blob/master/0_out.png"/><br/>
<img src="https://github.com/planet0104/edge_detection/blob/master/1.png"/><img src="https://github.com/planet0104/edge_detection/blob/master/1_out.png"/><br/><br/>
此代码并非专业用途，纯属个人爱好。如果此算法并没有什么实际用途或者本身有错误，欢迎指正。
