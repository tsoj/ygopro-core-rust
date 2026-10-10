//! Harpies' Hunting Ground
use super::prelude::*;

pub const ID: u32 = 75782277;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_HARPIE_LADY, CARD_HARPIE_LADY_SISTERS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // trigger
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_EVENT_PLAYER)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_EVENT_PLAYER, 0);
    // e2:SetCode(EVENT_CUSTOM+id)
    effect::set_code(d, e2, EVENT_CUSTOM + ID);
    // e2:SetTarget(s.target)
    effect::set_target(d, e2, Some(cost_target(target)));
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e2, Some(operation(op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // atk/def
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e3, EFFECT_UPDATE_ATTACK);
    // e3:SetRange(LOCATION_FZONE)
    effect::set_range(d, e3, LOCATION_FZONE);
    // e3:SetTarget(aux.TargetBoolFunction(Card.IsRace,RACE_WINGEDBEAST))
    effect::set_target(d, e3, Some(effect_card_fn(|d, _e, c| card::is_race(d, c, RACE_WINGEDBEAST))));
    // e3:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e3, LOCATION_MZONE, LOCATION_MZONE);
    // e3:SetValue(200)
    effect::set_value(d, e3, 200);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e4, EFFECT_UPDATE_DEFENSE);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // aux.GlobalCheck(s,function()
    aux::global_check(d, ID, |d| {
        // local ge1=Effect.CreateEffect(c)
        let ge1 = effect::create_effect(d, c);
        // ge1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // ge1:SetCode(EVENT_SUMMON_SUCCESS)
        effect::set_code(d, ge1, EVENT_SUMMON_SUCCESS);
        // ge1:SetOperation(s.check)
        effect::set_operation(d, ge1, Some(operation(check)));
        // Duel.RegisterEffect(ge1,0)
        duel::register_effect(d, ge1, 0);
        // local ge2=ge1:Clone()
        let ge2 = effect::clone(d, ge1);
        // ge2:SetCode(EVENT_SPSUMMON_SUCCESS)
        effect::set_code(d, ge2, EVENT_SPSUMMON_SUCCESS);
        // Duel.RegisterEffect(ge2,0)
        duel::register_effect(d, ge2, 0);
    });
}
/// `s.check(e,tp,eg,ep,ev,re,r,rp)`
fn check(d: &mut Duel, a: &EffArgs) {
    // local g1=Group.CreateGroup()
    let mut g1 = CardSet::new();
    // local g2=Group.CreateGroup()
    let mut g2 = CardSet::new();
    // for tc in aux.Next(eg) do
    let eg = a.eg.clone().nil();
    for tc in group::iter(d, &eg) {
        // if tc:IsFaceup() and tc:IsCode(CARD_HARPIE_LADY,CARD_HARPIE_LADY_SISTERS) then
        if card::is_faceup(d, tc) && card::is_code(d, tc, &[CARD_HARPIE_LADY, CARD_HARPIE_LADY_SISTERS]) {
            // if tc:IsControler(0) then g1:AddCard(tc) else g2:AddCard(tc) end
            if card::is_controler(d, tc, 0) {
                group::add_card(&mut g1, &CardSet::from_card(tc));
            } else {
                group::add_card(&mut g2, &CardSet::from_card(tc));
            }
        }
    }
    // if #g1>0 then Duel.RaiseEvent(g1,EVENT_CUSTOM+id,re,r,rp,0,0) end
    if !g1.is_empty() {
        duel::raise_event(d, g1, EVENT_CUSTOM + ID, a.re, a.r, a.rp, 0, 0);
    }
    // if #g2>0 then Duel.RaiseEvent(g2,EVENT_CUSTOM+id,re,r,rp,1,0) end
    if !g2.is_empty() {
        duel::raise_event(d, g2, EVENT_CUSTOM + ID, a.re, a.r, a.rp, 1, 0);
    }
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpellTrap()
    card::is_spell_trap(d, c)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsOnField() and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && filter(d, chkc);
    }
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&filter), a.tp, LOCATION_ONFIELD as u32, LOCATION_ONFIELD as u32, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) {
            // Duel.Destroy(tc,REASON_EFFECT)
            duel::destroy(d, tc, REASON_EFFECT, None, None);
        }
    }
}
