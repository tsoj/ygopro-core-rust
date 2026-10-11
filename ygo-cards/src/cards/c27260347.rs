//! Snake-Eyes Diabellstar
//! 蛇眼の大炎魔
use super::prelude::*;

pub const ID: u32 = 27260347;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Place battling monsters in the Spell/Trap Zone as Continuous Spells
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.btplcon)
    effect::set_condition(d, e1, Some(condition(btplcon)));
    // e1:SetOperation(s.btplop)
    effect::set_operation(d, e1, Some(operation(btplop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Place 1 monster from the GY in the Spell/Trap Zone as a Continuous Spell
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_LEAVE_GRAVE+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_LEAVE_GRAVE + CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function(e) return e:GetHandler():IsContinuousSpell() end)
    effect::set_condition(d, e2, Some(condition(gyplcon)));
    // e2:SetTarget(s.gypltg)
    effect::set_target(d, e2, Some(cost_target(gypltg)));
    // e2:SetOperation(s.gyplop)
    effect::set_operation(d, e2, Some(operation(gyplop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.checkzones(c0,c1)`
fn checkzones(d: &mut Duel, c0: CardId, c1: CardId) -> bool {
    // local p0,p1=c0:GetOwner(),c1:GetOwner()
    let p0 = card::get_owner(d, c0);
    let p1 = card::get_owner(d, c1);
    // if p0==p1 then return Duel.GetLocationCount(p0,LOCATION_SZONE)>1 end
    if p0 == p1 {
        return duel::get_location_count(d, p0, LOCATION_SZONE as u8, None, None, None).0 > 1;
    }
    // return Duel.GetLocationCount(p0,LOCATION_SZONE)>0 and Duel.GetLocationCount(p1,LOCATION_SZONE)>0
    duel::get_location_count(d, p0, LOCATION_SZONE as u8, None, None, None).0 > 0
        && duel::get_location_count(d, p1, LOCATION_SZONE as u8, None, None, None).0 > 0
}
/// `s.btplcon(e,tp,eg,ep,ev,re,r,rp)`
fn btplcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local bc0,bc1=Duel.GetBattleMonster(tp)
    let (bc0, bc1) = duel::get_battle_monster(d, a.tp);
    // return bc0 and bc1 and bc0==e:GetHandler() and s.checkzones(bc0,bc1)
    match (bc0, bc1) {
        (Some(bc0), Some(bc1)) => Some(bc0) == effect::get_handler(d, a.e) && checkzones(d, bc0, bc1),
        _ => false,
    }
}
/// `s.stplace(c,tp,rc)`
fn stplace(d: &mut Duel, c: CardId, tp: u8, rc: CardId) -> bool {
    // if not Duel.MoveToField(c,tp,c:GetOwner(),LOCATION_SZONE,POS_FACEUP,c:IsMonsterCard()) then return end
    let owner = card::get_owner(d, c);
    let mon = card::is_monster_card(d, c);
    if !duel::move_to_field(d, c, tp, owner, LOCATION_SZONE as u16, POS_FACEUP as u8, mon, None) {
        return false;
    }
    // Treated as a Continuous Spell
    // local e1=Effect.CreateEffect(rc)
    let e1 = effect::create_effect(d, rc);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e1:SetCode(EFFECT_CHANGE_TYPE)
    effect::set_code(d, e1, EFFECT_CHANGE_TYPE);
    // e1:SetValue(TYPE_SPELL|TYPE_CONTINUOUS)
    effect::set_value(d, e1, (TYPE_SPELL | TYPE_CONTINUOUS) as i64);
    // e1:SetReset(RESET_EVENT|(RESETS_STANDARD&~RESET_TURN_SET))
    effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD & !RESET_TURN_SET), 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // return true
    true
}
/// `s.btplop(e,tp,eg,ep,ev,re,r,rp)`
fn btplop(d: &mut Duel, a: &EffArgs) {
    // local bc0,bc1=Duel.GetBattleMonster(tp)
    let (bc0, bc1) = duel::get_battle_monster(d, a.tp);
    // if bc0 and bc1 and bc0:IsRelateToBattle() and not bc0:IsImmuneToEffect(e)
    //     and bc1:IsRelateToBattle() and not bc1:IsImmuneToEffect(e)
    //     and s.checkzones(bc0,bc1) and s.stplace(bc0,tp,bc0) then
    //     s.stplace(bc1,tp,bc0)
    // end
    if let (Some(bc0), Some(bc1)) = (bc0, bc1) {
        if card::is_relate_to_battle(d, bc0)
            && !card::is_immune_to_effect(d, bc0, a.e)
            && card::is_relate_to_battle(d, bc1)
            && !card::is_immune_to_effect(d, bc1, a.e)
            && checkzones(d, bc0, bc1)
            && stplace(d, bc0, a.tp, bc0)
        {
            stplace(d, bc1, a.tp, bc0);
        }
    }
}
/// `s.gyplfilter(c)`
fn gyplfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_FIRE) and not c:IsCode(id) and not c:IsForbidden()
    card::is_attribute(d, c, ATTRIBUTE_FIRE) && !card::is_code(d, c, &[ID]) && !card::is_forbidden(d, c)
}
/// `s.gyplcon`: `function(e) return e:GetHandler():IsContinuousSpell() end`
fn gyplcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_continuous_spell(d, c)
}
/// `s.gypltg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn gypltg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_GRAVE) and s.gyplfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_GRAVE) && gyplfilter(d, chkc);
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e);
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingTarget(s.gyplfilter,tp,LOCATION_GRAVE,0,1,nil)
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    // end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&gyplfilter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX)
            && card::is_can_be_special_summoned(d, c.nil(), a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // local g=Duel.SelectTarget(tp,s.gyplfilter,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&gyplfilter), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, g, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, c.map(CardSet::from_card), 1, 0, 0);
    true
}
/// `s.gyplop(e,tp,eg,ep,ev,re,r,rp)`
fn gyplop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e);
    // if tc:IsRelateToEffect(e) and not tc:IsImmuneToEffect(e)
    //     and s.stplace(tc,tp,c) and c:IsRelateToEffect(e) then
    //     Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
    // end
    if card::is_relate_to_effect(d, tc, a.e)
        && !card::is_immune_to_effect(d, tc, a.e)
        && stplace(d, tc, a.tp, c.nil())
        && card::is_relate_to_effect(d, c.nil(), a.e)
    {
        duel::special_summon(d, c.nil(), 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
