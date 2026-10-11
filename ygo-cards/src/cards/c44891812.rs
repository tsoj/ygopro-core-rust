//! Superheavy Samurai Helper
//! 超重武者オタス－K
use super::prelude::*;

pub const ID: u32 = 44891812;

// s.listed_series={SET_SUPERHEAVY_SAMURAI}
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SUPERHEAVY_SAMURAI], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // def up
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DEFCHANGE)
    effect::set_category(d, e1, CATEGORY_DEFCHANGE);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_PRE_DAMAGE_CALCULATE)
    effect::set_code(d, e1, EVENT_PRE_DAMAGE_CALCULATE);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCondition(s.defcon)
    effect::set_condition(d, e1, Some(condition(defcon)));
    // e1:SetCost(Cost.SelfDiscard)
    effect::set_cost(d, e1, Some(cost::self_discard()));
    // e1:SetTarget(s.deftg)
    effect::set_target(d, e1, Some(cost_target(deftg)));
    // e1:SetOperation(s.defop)
    effect::set_operation(d, e1, Some(operation(defop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // special summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e2, EVENT_ATTACK_ANNOUNCE);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(condition(spcon)));
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.defcon(e,tp,eg,ep,ev,re,r,rp)`
fn defcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetMatchingGroupCount(Card.IsSpellTrap,tp,LOCATION_GRAVE,0,nil)==0
    duel::get_matching_group_count(d, Some(&card::is_spell_trap), a.tp, LOCATION_GRAVE, 0, NO_EX) == 0
}
/// `s.deffilter(c)`
fn deffilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_SUPERHEAVY_SAMURAI) and c:HasNonZeroDefense()
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_SUPERHEAVY_SAMURAI as u16]) && card::has_non_zero_defense(d, c)
}
/// `s.deftg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn deftg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local a=Duel.GetAttacker()
    let atk = duel::get_attacker(d).nil();
    // local d=a:GetBattleTarget()   (the Lua local d is called dfn here, the duel is d)
    let mut at = Some(atk);
    let mut dfn = card::get_battle_target(d, atk);
    // if a:IsControler(1-tp) then a,d=d,a end
    if card::is_controler(d, atk, 1 - tp) {
        at = dfn;
        dfn = Some(atk);
    }
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp)
    //     and s.deffilter(chkc) and chkc~=e:GetLabelObject() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE)
            && card::is_controler(d, chkc, tp)
            && deffilter(d, chkc)
            && Some(chkc) != effect::get_label_object(d, a.e).as_card();
    }
    // if chk==0 then return a and a:IsDefensePos() and d and d:IsControler(1-tp)
    //     and Duel.IsExistingTarget(s.deffilter,tp,LOCATION_MZONE,0,1,a) end
    if a.chk == Some(0) {
        return match (at, dfn) {
            (Some(ac), Some(dc)) => {
                card::is_defense_pos(d, ac)
                    && card::is_controler(d, dc, 1 - tp)
                    && duel::is_existing_target(d, Some(&deffilter), tp, LOCATION_MZONE, 0, 1, ac.into())
            }
            _ => false,
        };
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,s.deffilter,tp,LOCATION_MZONE,0,1,1,a)
    let _ = duel::select_target(d, tp, Some(&deffilter), tp, LOCATION_MZONE, 0, 1, 1, false, at.into());
    // e:SetLabelObject(a)
    effect::set_label_object(d, a.e, at.map_or(Value::Nil, Value::Card));
    true
}
/// `s.defop(e,tp,eg,ep,ev,re,r,rp)`
fn defop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // local ac=e:GetLabelObject()
    let ac = effect::get_label_object(d, a.e).as_card().nil();
    // if ac:IsRelateToBattle() and ac:IsFaceup() and tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if card::is_relate_to_battle(d, ac)
        && card::is_faceup(d, ac)
        && card::is_relate_to_effect(d, tc.nil(), a.e)
        && card::is_faceup(d, tc.nil())
    {
        let tc = tc.nil();
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_DEFENSE)
        effect::set_code(d, e1, EFFECT_UPDATE_DEFENSE);
        // e1:SetValue(tc:GetDefense())
        let def = card::get_defense(d, tc);
        effect::set_value(d, e1, def as i64);
        // e1:SetReset(RESET_PHASE|PHASE_DAMAGE)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_DAMAGE, 1);
        // ac:RegisterEffect(e1)
        card::register_effect(d, ac, e1, false);
    }
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local at=Duel.GetAttacker()
    let at = duel::get_attacker(d).nil();
    // return at:GetControler()~=tp and Duel.GetAttackTarget()==nil
    card::get_controler(d, at) != a.tp && duel::get_attack_target(d).is_none()
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_SUPERHEAVY_SAMURAI) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_SUPERHEAVY_SAMURAI as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp)
    //     and s.spfilter(chkc,e,tp) and chkc~=c end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE)
            && card::is_controler(d, chkc, tp)
            && spfilter(d, chkc, a.e, tp)
            && chkc != c;
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingTarget(s.spfilter,tp,LOCATION_GRAVE,0,1,c,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, x| spfilter(d, x, a.e, tp)), tp, LOCATION_GRAVE, 0, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.spfilter,tp,LOCATION_GRAVE,0,1,1,c,e,tp)
    let g = duel::select_target(d, tp, Some(&|d, x| spfilter(d, x, a.e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, c.into()).nil();
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g), 1, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
