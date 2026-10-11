//! Supreme King's Castle
use super::prelude::*;

pub const ID: u32 = 72043279;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_DARK_FUSION], listed_series: &[SET_EVIL_HERO], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --Dark Fusion Ignore
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(id)
    effect::set_code(d, e2, ID);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetTargetRange(1,0)
    effect::set_target_range(d, e2, 1, 0);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // --atk & def
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e3, CATEGORY_ATKCHANGE);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_PRE_DAMAGE_CALCULATE)
    effect::set_code(d, e3, EVENT_PRE_DAMAGE_CALCULATE);
    // e3:SetRange(LOCATION_FZONE)
    effect::set_range(d, e3, LOCATION_FZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCondition(s.atkcon)
    effect::set_condition(d, e3, Some(condition(atkcon)));
    // e3:SetCost(s.atkcost)
    effect::set_cost(d, e3, Some(cost_target(atkcost)));
    // e3:SetOperation(s.atkop)
    effect::set_operation(d, e3, Some(operation(atkop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.atkcon(e,tp,eg,ep,ev,re,r,rp)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local tc=Duel.GetAttacker()
    let tc = duel::get_attacker(d);
    // local bc=Duel.GetAttackTarget()
    // if not bc then return false end
    let Some(mut b) = duel::get_attack_target(d) else {
        return false;
    };
    // if bc:IsControler(1-tp) then bc=tc end
    if card::is_controler(d, b, 1 - tp) {
        b = tc.nil();
    }
    // e:SetLabelObject(bc)
    effect::set_label_object(d, e, Value::Card(b));
    // return bc:IsFaceup() and bc:IsRace(RACE_FIEND)
    card::is_faceup(d, b) && card::is_race(d, b, RACE_FIEND)
}
/// `s.atkcfilter(c)`
fn atkcfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsSetCard(SET_EVIL_HERO) and c:HasLevel() and c:IsAbleToGraveAsCost()
    card::is_monster(d, c) && card::is_set_card(d, c, &[SET_EVIL_HERO as u16]) && card::has_level(d, c) && card::is_able_to_grave_as_cost(d, c)
}
/// `s.atkcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atkcost(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.IsExistingMatchingCard(s.atkcfilter,tp,LOCATION_DECK|LOCATION_EXTRA,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| atkcfilter(d, c)), tp, LOCATION_DECK | LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.atkcfilter,tp,LOCATION_DECK|LOCATION_EXTRA,0,1,1,nil):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&|d, c| atkcfilter(d, c)), tp, LOCATION_DECK | LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g).nil();
    // Duel.SendtoGrave(tc,REASON_COST)
    duel::send_to_grave(d, CardSet::from_card(tc), REASON_COST, None, None);
    // e:SetLabel(tc:GetLevel())
    let lv = card::get_level(d, tc);
    effect::set_label(d, e, &[lv as i64]);
    true
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, e).nil();
    if !card::is_relate_to_effect(d, h, e) {
        return;
    }
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, e).as_card().nil();
    // local ct=e:GetLabel()*200
    let ct = effect::get_label(d, e) * 200;
    // if tc:IsRelateToBattle() and tc:IsFaceup() and tc:IsControler(tp) then
    if card::is_relate_to_battle(d, tc) && card::is_faceup(d, tc) && card::is_controler(d, tc, tp) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(ct)
        effect::set_value(d, e1, ct);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
