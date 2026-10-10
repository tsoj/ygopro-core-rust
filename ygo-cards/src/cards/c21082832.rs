//! Chaos Form
use super::prelude::*;

pub const ID: u32 = 21082832;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_DARK_MAGICIAN, CARD_BLUEEYES_W_DRAGON],
    listed_series: &[SET_CHAOS, SET_NUMBER_C],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --Activate
    // Ritual.AddProcEqual{handler=c,filter=s.ritualfil,extrafil=s.extrafil,extratg=s.extratg}
    ritual::add_proc_equal(
        d,
        c,
        ritual::RitualProc {
            filter: Some(std::sync::Arc::new(|d: &mut Duel, c: CardId| ritualfil(d, c))),
            extrafil: Some(std::sync::Arc::new(extrafil)),
            extratg: Some(cost_target(extratg)),
            ..Default::default()
        },
    );
}
/// `s.ritualfil(c)`
fn ritualfil(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard({SET_CHAOS,SET_NUMBER_C}) and c:IsRitualMonster()
    card::is_set_card(d, c, &[SET_CHAOS as u16, SET_NUMBER_C as u16]) && card::is_ritual_monster(d, c)
}
/// `s.mfilter(c)`
fn mfilter(d: &mut Duel, c: CardId) -> bool {
    // return not Duel.IsPlayerAffectedByEffect(c:GetControler(),CARD_SPIRIT_ELIMINATION) and c:HasLevel() and c:IsCode(CARD_DARK_MAGICIAN,CARD_BLUEEYES_W_DRAGON)
    //     and c:IsMonster() and c:IsAbleToRemove()
    let p = card::get_controler(d, c);
    if duel::is_player_affected_by_effect(d, p, CARD_SPIRIT_ELIMINATION).is_some() {
        return false;
    }
    card::has_level(d, c) && card::is_code(d, c, &[CARD_DARK_MAGICIAN, CARD_BLUEEYES_W_DRAGON]) && card::is_monster(d, c) && card::is_able_to_remove(d, c, None, None, None)
}
/// `s.extrafil(e,tp,eg,ep,ev,re,r,rp,chk)`
fn extrafil(d: &mut Duel, a: &EffArgs) -> Option<CardSet> {
    // return Duel.GetMatchingGroup(s.mfilter,tp,LOCATION_GRAVE,0,nil)
    Some(duel::get_matching_group(d, Some(&mfilter), a.tp, LOCATION_GRAVE, 0, NO_EX))
}
/// `s.extratg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn extratg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
