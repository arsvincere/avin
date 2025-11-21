/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use avin_analyse::TrendAnalytic;
use avin_core::{Chart, ExtremumIndicator};
use avin_utils::AvinError;

use crate::{Condition, FilterResult, Marker, Point};

pub struct Filter {}
impl Filter {
    pub fn run(
        chart: &Chart,
        filter: impl Condition,
        marker: Marker,
    ) -> Result<(), AvinError> {
        // временный вектор для найденных точек, где фильтр сработал
        let mut points = Vec::new();

        // первое нужен пустой график того же актива и таймфрейма
        let mut new_chart = Chart::empty(chart.iid(), chart.tf());
        ExtremumIndicator::init(&mut new_chart);
        TrendAnalytic::init(&mut new_chart);

        // берем бары от переданного графика
        let bars = chart.bars();

        // добавляем эти бары поштучно в пустой график и чекаем фильтр
        for bar in bars.iter() {
            new_chart.add_bar(*bar);

            let result = filter.apply(&new_chart);

            if result {
                let ts = new_chart.now().unwrap().ts;
                let price = new_chart.now().unwrap().h * 1.003;
                points.push(Point::new(ts, price));
            }
        }

        // сохраняем результаты в файл
        let scan_result = FilterResult::new(chart, filter, marker, points);
        FilterResult::save(&scan_result)
    }
}
