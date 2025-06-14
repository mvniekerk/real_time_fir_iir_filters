use crate::{calc::iir::first::FirstOrderCalc, conf::{All, HighPass, LowPass}, param::{FirstOrderFilterConf, FirstOrderFilterParam, OmegaFirstOrder}, real_time_fir_iir_filters};

crate::def_rtf!(
    {
        /// # Configurations
        /// 
        /// [`All`](crate::conf::All), [`LowPass`](crate::conf::LowPass), [`HighPass`](crate::conf::HighPass)
        /// 
        /// <pre>
        /// 0) LOW-PASS:
        /// 
        ///          ω
        /// H(s) = -----
        ///        s + ω
        /// 
        /// 1) HIGH-PASS
        /// 
        ///          s
        /// H(s) = -----
        ///        s + ω
        /// </pre>
        /// 
        /// # Frequency response
        /// 
        /// ## Parameters
        /// 
        /// ω = 10 kHz 2π
        /// 
        /// ## Low-pass
        /// 
        /// <div>
        /// <img alt="First order low-pass filter response" src="https://raw.githubusercontent.com/sigurd4/real_time_fir_iir_filters/refs/heads/master/plots/first_order_filter0.png" height="500">
        /// </div>
        /// 
        /// ## High-pass
        /// 
        /// <div>
        /// <img alt="First order high-pass filter response" src="https://raw.githubusercontent.com/sigurd4/real_time_fir_iir_filters/refs/heads/master/plots/first_order_filter1.png" height="500">
        /// </div>
    }
    FirstOrderFilter
    {
        type Conf: FirstOrderFilterConf;
        type Param: FirstOrderFilterParam = OmegaFirstOrder;

        const O_BUFFERS: usize = 1;
        const SOS_BUFFERS: usize = 1;
        const SOS_STAGES: usize = 0;
        const ORDER: usize = 1;
        const IS_IIR: bool = true;

        fn make_coeffs<All>(param, rate) -> _
        {
            let calc = FirstOrderCalc::new(param.omega(), rate);
            (
                ([], [], [
                    calc.b_low(),
                    calc.b_high()
                ]),
                [([], [
                    calc.a()
                ])]
            )
        }
        fn make_coeffs<LowPass>(param, rate) -> _
        {
            let calc = FirstOrderCalc::new(param.omega(), rate);
            (
                ([], [], [
                    calc.b_low()
                ]),
                [([], [
                    calc.a()
                ])]
            )
        }
        fn make_coeffs<HighPass>(param, rate) -> _
        {
            let calc = FirstOrderCalc::new(param.omega(), rate);
            (
                ([], [], [
                    calc.b_high()
                ]),
                [([], [
                    calc.a()
                ])]
            )
        }
    }
    where
        [(); <C as FirstOrderFilterConf>::OUTPUTS]:
);

#[cfg(test)]
mod test
{
    use core::f64::consts::TAU;

    use super::FirstOrderFilter;

    use crate::{conf::All, param::Omega};

    #[test]
    fn plot()
    {
        let mut filter = FirstOrderFilter::<All>::new(Omega {omega: 10e3*TAU});
        crate::tests::plot_freq(&mut filter, false).unwrap();
    }
}