use std::vec::Vec;

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum ShaderDataType {
    Float,
    Float2,
    Float3,
    Float4,
    Mat3,
    Mat4,
    Int,
    Int2,
    Int3,
    Int4,
    Bool,
}

fn get_shader_type_size(type_: ShaderDataType) -> usize {
    match type_ {
        ShaderDataType::Float => 4,
        ShaderDataType::Float2 => 4 * 2,
        ShaderDataType::Float3 => 4 * 3,
        ShaderDataType::Float4 => 4 * 4,
        ShaderDataType::Mat3 => 4 * 3 * 3,
        ShaderDataType::Mat4 => 4 * 4 * 4,
        ShaderDataType::Int | ShaderDataType::Bool => 4,
        ShaderDataType::Int2 => 4 * 2,
        ShaderDataType::Int3 => 4 * 3,
        ShaderDataType::Int4 => 4 * 4,
    }
}

#[derive(Debug, Copy, Clone)]
pub struct BufferElement {
    pub component_count: u32,
    pub shader_type: ShaderDataType,
    pub gl_type: gl::types::GLenum,
    pub is_normalized: bool,
    pub offset: usize,
}

impl BufferElement {
    pub fn new(type_: ShaderDataType, normalized: bool) -> Self {
        let component_count = match type_ {
            ShaderDataType::Float | ShaderDataType::Int | ShaderDataType::Bool => 1,
            ShaderDataType::Float2 | ShaderDataType::Int2 => 2,
            ShaderDataType::Float3 | ShaderDataType::Int3 => 3,
            ShaderDataType::Float4 | ShaderDataType::Int4 => 4,
            ShaderDataType::Mat3 => 3 * 3,
            ShaderDataType::Mat4 => 4 * 4,
        };
        let gl_type = match type_ {
            ShaderDataType::Float
            | ShaderDataType::Float2
            | ShaderDataType::Float3
            | ShaderDataType::Float4
            | ShaderDataType::Mat3
            | ShaderDataType::Mat4 => gl::FLOAT,
            ShaderDataType::Int
            | ShaderDataType::Int2
            | ShaderDataType::Int3
            | ShaderDataType::Int4 => gl::INT,
            ShaderDataType::Bool => gl::BOOL,
        };
        Self {
            component_count,
            shader_type: type_,
            gl_type,
            is_normalized: normalized,
            offset: 0,
        }
    }

    pub fn shader_type(&self) -> ShaderDataType {
        self.shader_type
    }
}

#[derive(Debug)]
pub struct BufferLayout {
    m_elements: Vec<BufferElement>,
    m_stride: usize,
}

impl BufferLayout {
    pub fn new(elements: &[BufferElement]) -> Self {
        let mut layout = Self {
            m_elements: elements.to_vec(),
            m_stride: 0,
        };
        layout.calc_stride_and_offsets();
        layout
    }

    pub fn get_stride(&self) -> usize {
        self.m_stride
    }

    pub fn push_back(&mut self, element: BufferElement) {
        self.m_elements.push(element);
        self.calc_stride_and_offsets();
    }

    pub fn clear(&mut self) {
        self.m_elements.clear();
        self.m_stride = 0;
    }

    pub fn empty(&self) -> bool {
        self.m_elements.is_empty()
    }

    pub fn get_elements(&self) -> &[BufferElement] {
        &self.m_elements
    }

    pub fn iter(&self) -> std::slice::Iter<'_, BufferElement> {
        self.m_elements.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, BufferElement> {
        self.m_elements.iter_mut()
    }

    fn calc_stride_and_offsets(&mut self) {
        self.m_stride = 0;
        let mut offset: usize = 0;
        for element in &mut self.m_elements {
            element.offset = offset;
            let size = get_shader_type_size(element.shader_type);
            offset += size;
            self.m_stride += size;
        }
    }
}
