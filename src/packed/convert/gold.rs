use crate::core::traits::{ImageView, Pixel, TryConvert, TryConvertSlice};
use crate::packed::generic::{GenericBuffer, GenericFlatBuffer, GenericView};
use crate::packed::traits::{AccessPixel, AccessPixelMut};

macro_rules! impl_TryConvert {
    () => {
        type Error = ();

        fn try_convert(&self, output: &mut GenericBuffer<DP>) -> Result<(), Self::Error> {
            if output.width() < self.width() || output.height() < self.height() {
                *output = GenericBuffer::new(self.width(), self.height());
            }

            // iterate over the source pixels and convert them
            for i in 0..self.height() {
                let row_in = self.row(i).unwrap();
                let row_out = output.row_mut(i).unwrap();
                let res = SP::try_convert(row_in, row_out);
                if res.is_err() {
                    return Err(())
                }
            }

            Ok(())
        }
    };
}

macro_rules! impl_TryConvertFlat {
    () => {
        type Error = ();

        fn try_convert(&self, output: &mut GenericFlatBuffer<DP>) -> Result<(), Self::Error> {
            if output.width() < self.width() || output.height() < self.height() {
                return Err(());
            }

            // iterate over the source pixels and convert them
            for i in 0..self.height() {
                let row_in = self.row(i).unwrap();
                let row_out = output.row_mut(i).unwrap();
                let res = SP::try_convert(row_in, row_out);
                if res.is_err() {
                    return Err(())
                }
            }

            Ok(())
        }
    };
}

impl<'a, SP, DP> TryConvert<GenericBuffer<DP>> for GenericView<'a, SP>
where
    SP: Pixel,
    DP: Pixel,
    SP: TryConvertSlice<DP>,
{
    impl_TryConvert!();
}

impl<'a, SP, DP> TryConvert<GenericBuffer<DP>> for GenericFlatBuffer<'a, SP>
where
    SP: Pixel,
    DP: Pixel,
    SP: TryConvertSlice<DP>,
{
    impl_TryConvert!();
}

impl<SP, DP> TryConvert<GenericBuffer<DP>> for GenericBuffer<SP>
where
    SP: Pixel,
    DP: Pixel,
    SP: TryConvertSlice<DP>,
{
    impl_TryConvert!();
}

impl<'a, SP, DP> TryConvert<GenericFlatBuffer<'a, DP>> for GenericView<'a, SP>
where
    SP: Pixel,
    DP: Pixel,
    SP: TryConvertSlice<DP>,
{
    impl_TryConvertFlat!();
}

impl<'a, SP, DP> TryConvert<GenericFlatBuffer<'a, DP>> for GenericFlatBuffer<'a, SP>
where
    SP: Pixel,
    DP: Pixel,
    SP: TryConvertSlice<DP>,
{
    impl_TryConvertFlat!();
}

impl<'a, SP, DP> TryConvert<GenericFlatBuffer<'a, DP>> for GenericBuffer<SP>
where
    SP: Pixel,
    DP: Pixel,
    SP: TryConvertSlice<DP>,
{
    impl_TryConvertFlat!();
}
