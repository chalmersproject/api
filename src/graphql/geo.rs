use super::prelude::*;
use service::Coordinate as CoordinateRepr;

#[derive(Debug, Clone)]
pub struct Coordinate(CoordinateRepr);

impl From<CoordinateRepr> for Coordinate {
    fn from(coordinate: CoordinateRepr) -> Self {
        Self(coordinate)
    }
}

impl From<Coordinate> for CoordinateRepr {
    fn from(coordinate: Coordinate) -> Self {
        coordinate.0
    }
}

/// A `Coordinate` represents a point on Earth by its longitude and latitude.
#[Scalar]
impl ScalarType for Coordinate {
    fn parse(value: Value) -> InputValueResult<Self> {
        let data = if let Value::List(data) = value {
            data
        } else {
            let error = InputValueError::expected_type(value);
            return Err(error);
        };

        let data = data
            .into_iter()
            .map(|value| {
                let x = if let Value::Number(x) = value {
                    x
                } else {
                    bail!("found non-numeric coordinate {}", &value);
                };
                let f = if let Some(f) = x.as_f64() {
                    f
                } else {
                    bail!("cannot represent coordinate {} as a float", &x);
                };
                Ok(f as f32)
            })
            .collect::<Result<Vec<_>>>()
            .map_err(InputValueError::custom)?;

        let coordinate = if let [x, y] = *data.as_slice() {
            CoordinateRepr { x, y }
        } else {
            let error = format!("expected 2 coordinates, found {}", data.len());
            return Err(InputValueError::custom(error));
        };
        Ok(coordinate.into())
    }

    fn to_value(&self) -> Value {
        let CoordinateRepr { x, y } = self.0;
        let data = vec![x, y];
        let data = data
            .into_iter()
            .map(|f| {
                let n = JsonNumber::from_f64(f as f64)
                    .expect("coordinates are always finite");
                Value::Number(n)
            })
            .collect::<Vec<_>>();
        Value::List(data)
    }
}
