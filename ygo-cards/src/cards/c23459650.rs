//! Rebirth of Nephthys
use super::prelude::*;

pub const ID: u32 = 23459650;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_NEPHTHYS], fit_monster: &[88176533, 24175232], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Ritual.AddProcGreater({handler=c,filter=s.ritualfil,stage2=s.stage2})
    ritual::add_proc_greater(
        d,
        c,
        ritual::RitualProc {
            filter: Some(std::sync::Arc::new(|d: &mut Duel, c: CardId| ritualfil(d, c))),
            stage2: Some(std::sync::Arc::new(|d: &mut Duel, mat: &CardSet, a: &EffArgs, tc: CardId| stage2(d, mat, a, tc))),
            ..Default::default()
        },
    );
}
// function s.ritualfil(c)
fn ritualfil(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_NEPHTHYS) and c:IsRitualMonster()
    card::is_set_card(d, c, &[SET_NEPHTHYS as u16]) && card::is_ritual_monster(d, c)
}
// function s.mfilter(c)
fn mfilter(d: &mut Duel, c: CardId) -> bool {
    // if c:IsPreviousLocation(LOCATION_MZONE) then
    if card::is_previous_location(d, c, LOCATION_MZONE) {
        // local code=c:GetPreviousCodeOnField()
        let code = card::get_previous_code_on_field(d, c).first().copied();
        // return code==88176533 or code==24175232
        return code == Some(88176533) || code == Some(24175232);
    }
    // return c:IsCode(88176533,24175232)
    card::is_code(d, c, &[88176533, 24175232])
}
// function s.stage2(mg,e,tp,eg,ep,ev,re,r,rp,tc)
fn stage2(d: &mut Duel, mg: &CardSet, a: &EffArgs, _tc: CardId) {
    // local g=Duel.GetMatchingGroup(aux.TRUE,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let g = duel::get_matching_group(d, None, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, h.into());
    // if mg:IsExists(s.mfilter,1,nil) and #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
    if group::is_exists(d, mg, &mfilter, 1, NO_EX) && !g.is_empty() && duel::select_yes_no(d, a.tp, aux::stringid(ID, 0)) {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
        // local sg=g:Select(tp,1,1,e:GetHandler())
        let sg = group::select(d, &g, a.tp, 1, 1, false, h.into()).nil();
        // Duel.Destroy(sg,REASON_EFFECT)
        duel::destroy(d, sg, REASON_EFFECT, None, None);
    }
}
