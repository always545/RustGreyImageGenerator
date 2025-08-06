# RustGreyImageGenerator
this is a GreyImageGenerater written in rust.

# HOW TO USE IT?
Compile the source code by ur self,and use the executable program
like

RCIT \<mode> \<format> \<target> \<destination>

> **mode** :trans a image or a directory

> **format**:8-bit GreyImage or 16-bit GreyImage(**not implemented for directory yet**)

> **target**: path to the to-be translated img or dir

> **destination**:where to store translated img



# 一个使用rust编写的灰度图像转换器

使用方法如上，可以使用参数-help来简单的查看，这里只提供了源代码，如需要使用可以自己编译
，未来打算加入字符集映射功能，使用**yaml**文件配置来快速的生成字符画，还未加入对于文件夹的转换16位灰度图支持。

