//! Shinobird's Calling
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 73055622;

pub const INFO: ScriptInfo = ScriptInfo {
    fit_monster: &[25415052, 52900000],
    listed_names: &[25415052, 52900000],
    listed_card_types: &[TYPE_SPIRIT],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // Ritual.AddProcGreater{handler=c,filter=s.ritualfil,extrafil=s.extrafil,extratg=s.extratg}
    ritual::add_proc_greater(
        d,
        c,
        ritual::RitualProc {
            filter: Some(Arc::new(|d: &mut Duel, x: CardId| ritualfil(d, x))),
            extrafil: Some(Arc::new(|d: &mut Duel, a: &EffArgs| extrafil(d, a))),
            extratg: Some(cost_target(extratg)),
            ..Default::default()
        },
    );
}
// function s.ritualfil(c)
fn ritualfil(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(25415052,52900000) and c:IsRitualMonster()
    card::is_code(d, c, &[25415052, 52900000]) && card::is_ritual_monster(d, c)
}
// function s.mfilter(c)
fn mfilter(d: &mut Duel, c: CardId) -> bool {
    // return not Duel.IsPlayerAffectedByEffect(c:GetControler(),CARD_SPIRIT_ELIMINATION)
    //     and c:HasLevel() and c:IsType(TYPE_SPIRIT) and c:IsAbleToRemove()
    let controler = card::get_controler(d, c);
    duel::is_player_affected_by_effect(d, controler, CARD_SPIRIT_ELIMINATION).is_none()
        && card::has_level(d, c)
        && card::is_type(d, c, TYPE_SPIRIT)
        && card::is_able_to_remove(d, c, None, None, None)
}
// function s.extrafil(e,tp,eg,ep,ev,re,r,rp,chk)
fn extrafil(d: &mut Duel, a: &EffArgs) -> Option<CardSet> {
    // return Duel.GetMatchingGroup(s.mfilter,tp,LOCATION_GRAVE,0,nil)
    Some(duel::get_matching_group(d, Some(&mfilter), a.tp, LOCATION_GRAVE, 0, NO_EX))
}
// function s.extratg(e,tp,eg,ep,ev,re,r,rp,chk)
fn extratg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_REMOVE,nil,1,tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_REMOVE, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
