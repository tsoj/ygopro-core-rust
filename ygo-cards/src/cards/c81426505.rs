//! Six Strike - Triple Impact
use super::prelude::*;

pub const ID: u32 = 81426505;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SIX_SAMURAI], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCondition(s.descon)
    effect::set_condition(d, e1, Some(condition(descon)));
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_SIX_SAMURAI),tp,LOCATION_MZONE,0,3,nil)
    duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_SIX_SAMURAI as u16]))), a.tp, LOCATION_MZONE, 0, 3, NO_EX)
}
/// `s.filter2(c)`
fn filter2(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_spell_trap(d, c)
}
/// The group of `s.destg`/`s.desop` for the chosen option `opt` (0: monsters, 1: face-up
/// Spells/Traps, 2: face-down Spells/Traps), written out as the Lua does in each.
fn option_group(d: &mut Duel, tp: u8, opt: i64) -> CardSet {
    if opt == 0 {
        duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), tp, 0, LOCATION_MZONE, NO_EX)
    } else if opt == 1 {
        duel::get_matching_group(d, Some(&filter2), tp, 0, LOCATION_ONFIELD, NO_EX)
    } else {
        duel::get_matching_group(d, Some(&|d, c| card::is_facedown(d, c)), tp, 0, LOCATION_SZONE, NO_EX)
    }
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return ... end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c)), tp, 0, LOCATION_MZONE, 1, NO_EX)
            || duel::is_existing_matching_card(d, Some(&filter2), tp, 0, LOCATION_ONFIELD, 1, NO_EX)
            || duel::is_existing_matching_card(d, Some(&|d, c| card::is_facedown(d, c)), tp, 0, LOCATION_SZONE, 1, NO_EX);
    }
    // local t={}
    let mut t: Vec<u64> = Vec::new();
    // if Duel.IsExistingMatchingCard(Card.IsFaceup,tp,0,LOCATION_MZONE,1,nil) then t[p]=aux.Stringid(id,0) p=p+1 end
    if duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c)), tp, 0, LOCATION_MZONE, 1, NO_EX) {
        t.push(aux::stringid(ID, 0));
    }
    // if Duel.IsExistingMatchingCard(s.filter2,tp,0,LOCATION_ONFIELD,1,nil) then t[p]=aux.Stringid(id,1) p=p+1 end
    if duel::is_existing_matching_card(d, Some(&filter2), tp, 0, LOCATION_ONFIELD, 1, NO_EX) {
        t.push(aux::stringid(ID, 1));
    }
    // if Duel.IsExistingMatchingCard(Card.IsFacedown,tp,0,LOCATION_SZONE,1,nil) then t[p]=aux.Stringid(id,2) p=p+1 end
    if duel::is_existing_matching_card(d, Some(&|d, c| card::is_facedown(d, c)), tp, 0, LOCATION_SZONE, 1, NO_EX) {
        t.push(aux::stringid(ID, 2));
    }
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,3))
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 3));
    // local sel=Duel.SelectOption(tp,table.unpack(t))+1
    let sel = duel::select_option(d, tp, true, &t) as usize;
    // local opt=t[sel]-aux.Stringid(id,0)
    let opt = (t[sel] - aux::stringid(ID, 0)) as i64;
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,sg,#sg,0,0)
    let sg = option_group(d, tp, opt);
    let n = sg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(sg), n, 0, 0);
    // e:SetLabel(opt)
    effect::set_label(d, a.e, &[opt]);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local opt=e:GetLabel()
    let opt = effect::get_label(d, a.e);
    let sg = option_group(d, a.tp, opt);
    // Duel.Destroy(sg,REASON_EFFECT)
    duel::destroy(d, sg, REASON_EFFECT, None, None);
}
