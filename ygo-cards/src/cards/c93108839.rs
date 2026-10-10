//! No.58 炎圧鬼バーナー・バイサー
use super::prelude::*;

pub const ID: u32 = 93108839;

pub const INFO: ScriptInfo = ScriptInfo { xyz_number: Some(58), ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Xyz.AddProcedure(c,nil,4,2)
    xyz::add_procedure(d, c, None, Some(4), 2, None, None, None, None, false, None);
    // Xyz Summon Procedure
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Equip itself to an Xyz monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e1, CATEGORY_EQUIP);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTarget(s.eqtg)
    effect::set_target(d, e1, Some(cost_target(eqtg)));
    // e1:SetOperation(s.eqop)
    effect::set_operation(d, e1, Some(operation(eqop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special summon itself if it is equipped to a monster
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCondition(function(e) return e:GetHandler():GetEquipTarget() end)
    effect::set_condition(
        d,
        e2,
        Some(condition(|d, a| {
            let c = effect::get_handler(d, a.e).nil();
            card::get_equip_target(d, c).is_some()
        })),
    );
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Equipped monster can attack directly
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e3, EFFECT_TYPE_EQUIP);
    // e3:SetCode(EFFECT_DIRECT_ATTACK)
    effect::set_code(d, e3, EFFECT_DIRECT_ATTACK);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Inflict 500 Damage to the opponent
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e4, aux::stringid(ID, 2));
    // e4:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e4, CATEGORY_DAMAGE);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e4:SetCode(EVENT_BATTLE_DAMAGE)
    effect::set_code(d, e4, EVENT_BATTLE_DAMAGE);
    // e4:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e4, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e4:SetRange(LOCATION_SZONE)
    effect::set_range(d, e4, LOCATION_SZONE);
    // e4:SetCondition(s.damcon)
    effect::set_condition(d, e4, Some(condition(damcon)));
    // e4:SetCost(s.damcost)
    effect::set_cost(d, e4, Some(cost_target(damcost)));
    // e4:SetTarget(s.damtg)
    effect::set_target(d, e4, Some(cost_target(damtg)));
    // e4:SetOperation(s.damop)
    effect::set_operation(d, e4, Some(operation(damop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_XYZ)
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_XYZ)
}
/// The flag reset of `c:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD&~(RESET_LEAVE|RESET_TOFIELD)|RESET_PHASE|PHASE_END,0,1)`.
const FLAG_RESET: u32 = RESET_EVENT | (RESETS_STANDARD & !(RESET_LEAVE | RESET_TOFIELD)) | RESET_PHASE | PHASE_END;
/// `s.eqtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.filter(chkc) and chkc~=c end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && filter(d, chkc) && chkc != c;
    }
    // if chk==0 then return c:GetFlagEffect(id)==0
    //     and Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //     and Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,0,1,c) end
    if a.chk == Some(0) {
        return card::get_flag_effect(d, c, ID) == 0
            && duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&filter), tp, LOCATION_MZONE, 0, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EQUIP as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,0,1,1,c)
    let g = duel::select_target(d, tp, Some(&filter), tp, LOCATION_MZONE, 0, 1, 1, false, c.into()).nil();
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, Some(g), 1, 0, 0);
    // c:RegisterFlagEffect(id,...)
    card::register_flag_effect(d, c, ID, FLAG_RESET, 0, 1, 0, 0);
    true
}
/// `s.eqop(e,tp,eg,ep,ev,re,r,rp)`
fn eqop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if not c:IsRelateToEffect(e) or c:IsFacedown() then return end
    if !card::is_relate_to_effect(d, c, a.e) || card::is_facedown(d, c) {
        return;
    }
    // if not tc:IsRelateToEffect(e) or tc:IsFacedown() or tc:IsControler(1-tp) then
    let tc = tc.nil();
    if !card::is_relate_to_effect(d, tc, a.e) || card::is_facedown(d, tc) || card::is_controler(d, tc, 1 - tp) {
        // Duel.SendtoGrave(c,REASON_EFFECT)
        duel::send_to_grave(d, c, REASON_EFFECT, None, None);
        return;
    }
    // if not Duel.Equip(tp,c,tc,false) then return end
    if !duel::equip(d, tp, c, tc, false, false) {
        return;
    }
    // Equip limit
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    // e0:SetCode(EFFECT_EQUIP_LIMIT)
    effect::set_code(d, e0, EFFECT_EQUIP_LIMIT);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e0:SetValue(function(e,c) return c==e:GetLabelObject() end)
    effect::set_value_fn(d, e0, effect_card_fn(|d, e, c| effect::get_label_object(d, e).as_card() == Some(c)));
    // e0:SetLabelObject(tc)
    effect::set_label_object(d, e0, Value::Card(tc));
    // e0:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e0, RESET_EVENT | RESETS_STANDARD, 1);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:GetFlagEffect(id)==0
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return card::get_flag_effect(d, c, ID) == 0
            && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    // c:RegisterFlagEffect(id,...)
    card::register_flag_effect(d, c, ID, FLAG_RESET, 0, 1, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SpecialSummon(c,0,tp,tp,true,false,POS_FACEUP_ATTACK)
        duel::special_summon(d, c, 0, a.tp, a.tp, true, false, POS_FACEUP_ATTACK as u8, None);
    }
}
/// `s.damcon(e,tp,eg,ep,ev,re,r,rp)`
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return ep==1-tp and eg:GetFirst()==e:GetHandler():GetEquipTarget()
    if a.ep != 1 - a.tp {
        return false;
    }
    let first = group::get_first(d, &a.eg.clone().nil());
    let c = effect::get_handler(d, a.e).nil();
    first == card::get_equip_target(d, c)
}
/// `s.damcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn damcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsDiscardable,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_discardable(d, c, None, None)), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.DiscardHand(tp,Card.IsDiscardable,1,1,REASON_COST|REASON_DISCARD)
    duel::discard_hand(d, a.tp, Some(&|d, c| card::is_discardable(d, c, None, None)), 1, 1, REASON_COST | REASON_DISCARD, NO_EX);
    true
}
/// `s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - a.tp);
    // Duel.SetTargetParam(500)
    duel::set_target_param(d, 500);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,500)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, 500);
    true
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Damage(p,d,REASON_EFFECT)
    duel::damage(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
}
