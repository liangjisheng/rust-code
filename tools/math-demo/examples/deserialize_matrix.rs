// 本实例实现将矩阵序列化为 JSON，以及从 JSON 反序列化出矩阵。序列化由
// serde_json::to_string 处理，serde_json::from_str 则执行反序列化。

use nalgebra::DMatrix;

fn main() -> Result<(), std::io::Error> {
    let row_slice: Vec<i32> = (1..5001).collect();
    let matrix = DMatrix::from_row_slice(50, 100, &row_slice);

    // 序列化矩阵
    let serialized_matrix = serde_json::to_string(&matrix)?;

    // 反序列化出矩阵
    let deserialized_matrix: DMatrix<i32> = serde_json::from_str(&serialized_matrix)?;

    // 验证反序列化出的矩阵 `deserialized_matrix` 等同于原始矩阵 `matrix`
    assert_eq!(deserialized_matrix, matrix);

    Ok(())
}
