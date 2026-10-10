//! Red-Eyes Transmigration
use super::prelude::*;

pub const ID: u32 = 45410988;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_RED_EYES],
    fit_monster: &[19025379],
    listed_names: &[19025379],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // Ritual.AddProcGreater{handler=c,filter=s.ritualfil,lv=8,extrafil=s.extrafil,extratg=s.extratg}
    ritual::add_proc_greater(
        d,
        c,
        ritual::RitualProc {
            filter: Some(std::sync::Arc::new(|d: &mut Duel, c: CardId| ritualfil(d, c))),
            lv: Some(ritual::Lv::N(8)),
            extrafil: Some(std::sync::Arc::new(extrafil)),
            extratg: Some(cost_target(extratg)),
            ..Default::default()
        },
    );
}
fn ritualfil(d: &mut Duel, c: CardId) -> bool {
    card::is_code(d, c, &[19025379]) && card::is_ritual_monster(d, c)
}
fn mfilter(d: &mut Duel, c: CardId) -> bool {
    duel::is_player_affected_by_effect(d, card::get_controler(d, c), CARD_SPIRIT_ELIMINATION).is_none()
        && card::has_level(d, c)
        && card::is_set_card(d, c, &[SET_RED_EYES as u16])
        && card::is_monster(d, c)
        && card::is_able_to_remove(d, c, None, None, None)
}
/// `s.extrafil(e,tp,eg,ep,ev,re,r,rp,chk)`
fn extrafil(d: &mut Duel, a: &EffArgs) -> Option<CardSet> {
    Some(duel::get_matching_group(d, Some(&mfilter), a.tp, LOCATION_GRAVE, 0, NO_EX))
}
/// `s.extratg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn extratg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
