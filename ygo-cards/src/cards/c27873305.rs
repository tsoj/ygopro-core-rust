//! D/D/D Wave Oblivion King Caesar Ragnarok
//! DDD怒濤壊薙王カエサル・ラグナロク
use super::prelude::*;

pub const ID: u32 = 27873305;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_DD, SET_DARK_CONTRACT, SET_DDD],
    material_setcode: &[SET_DD, SET_DDD],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMixN(c,true,true,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_DDD),2)
    fusion::add_proc_mix_n(d, c, true, true, vec![(fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_set_card_ctx(d, c, &[SET_DDD as u16], Some(fc), st, Some(tp)))), 2)]);
    // tohand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_EQUIP)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_EQUIP);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // aux.AddEREquipLimit(c,nil,function(ec,_,tp) return ec:IsControler(1-tp) end,s.equipop,e1)
    aux::add_er_equip_limit(
        d,
        c,
        None,
        lua_fn(|d, a| card::is_controler(d, a.c(0), 1 - a.p(2))),
        lua_fn(|d, a| equipop(d, a.c(0), a.e(1), a.p(2), a.c(3))),
        e1,
        None,
        None,
        None,
    );
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return (Duel.GetAttacker()==c or Duel.GetAttackTarget()==c)
    duel::get_attacker(d) == Some(c) || duel::get_attack_target(d) == Some(c)
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and (c:IsSetCard(SET_DD) or c:IsSetCard(SET_DARK_CONTRACT)) and c:IsAbleToHand()
    card::is_faceup(d, c) && (card::is_set_card(d, c, &[SET_DD as u16]) || card::is_set_card(d, c, &[SET_DARK_CONTRACT as u16])) && card::is_able_to_hand(d, c, None)
}
/// `s.eqfilter(c)`
fn eqfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsAbleToChangeControler()
    card::is_faceup(d, c) && card::is_able_to_change_controler(d, c)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local h=e:GetHandler()
    let h = effect::get_handler(d, a.e).nil();
    // if chkc then return chkc:IsOnField() and chkc:IsControler(tp) and s.thfilter(chkc) and chkc~=e:GetHandler() end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_controler(d, chkc, a.tp) && thfilter(d, chkc) && chkc != h;
    }
    // if chk==0 then return Duel.IsExistingTarget(s.thfilter,tp,LOCATION_ONFIELD,0,1,e:GetHandler())
    //     and Duel.IsExistingMatchingCard(s.eqfilter,tp,0,LOCATION_MZONE,1,e:GetHandler():GetBattleTarget()) end
    if a.chk == Some(0) {
        let bt = card::get_battle_target(d, h);
        return duel::is_existing_target(d, Some(&thfilter), a.tp, LOCATION_ONFIELD, 0, 1, h.into())
            && duel::is_existing_matching_card(d, Some(&eqfilter), a.tp, 0, LOCATION_MZONE, 1, bt.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RTOHAND as u64);
    // local g1=Duel.SelectTarget(tp,s.thfilter,tp,LOCATION_ONFIELD,0,1,1,e:GetHandler())
    let g1 = duel::select_target(d, a.tp, Some(&thfilter), a.tp, LOCATION_ONFIELD, 0, 1, 1, false, h.into());
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g1,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, g1, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,nil,1,1-tp,LOCATION_MZONE)
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, None, 1, 1 - a.tp, LOCATION_MZONE as i32);
    true
}
/// `s.equipop(c,e,tp,tc)`
fn equipop(d: &mut Duel, c: CardId, e: EffectId, tp: u8, tc: CardId) {
    // local atk=tc:GetTextAttack()
    let mut atk = card::get_text_attack(d, tc);
    // if atk<0 then atk=0 end
    if atk < 0 {
        atk = 0;
    }
    // if not c:EquipByEffectAndLimitRegister(e,tp,tc) then return end
    if !card::equip_by_effect_and_limit_register(d, c, e, tp, tc, None, None) {
        return;
    }
    // if atk>0 then
    if atk > 0 {
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_EQUIP)
        effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
        // e2:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE+EFFECT_FLAG_OWNER_RELATE)
        effect::set_property(d, e2, EFFECT_FLAG_IGNORE_IMMUNE + EFFECT_FLAG_OWNER_RELATE, 0);
        // e2:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
        // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
        // e2:SetValue(atk)
        effect::set_value(d, e2, atk as i64);
        // tc:RegisterEffect(e2)
        card::register_effect(d, tc, e2, false);
    }
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) and Duel.SendtoHand(tc,nil,REASON_EFFECT)~=0 then
    let Some(tc) = tc else {
        return;
    };
    if !card::is_relate_to_effect(d, tc, a.e) || duel::send_to_hand(d, CardSet::from_card(tc), None, REASON_EFFECT, None) == 0 {
        return;
    }
    // Duel.ConfirmCards(1-tp,tc)
    duel::confirm_cards(d, 1 - a.tp, CardSet::from_card(tc));
    // local bc=c:GetBattleTarget()
    let bc = card::get_battle_target(d, c);
    // if c:IsFaceup() and c:IsRelateToEffect(e) and bc:IsRelateToBattle() then
    if card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) && card::is_relate_to_battle(d, bc.nil()) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
        // local g=Duel.SelectMatchingCard(tp,s.eqfilter,tp,0,LOCATION_MZONE,1,1,bc)
        let g = duel::select_matching_card(d, a.tp, Some(&eqfilter), a.tp, 0, LOCATION_MZONE, 1, 1, false, bc.into()).nil();
        // local ec=g:GetFirst()
        // if not ec then return end
        let Some(ec) = group::get_first(d, &g) else {
            return;
        };
        // s.equipop(c,e,tp,ec)
        equipop(d, c, a.e, a.tp, ec);
    }
}
