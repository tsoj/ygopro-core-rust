//! Arcana Force EX - The Light Ruler
use super::prelude::*;

pub const ID: u32 = 5861892;
pub const INFO: ScriptInfo = ScriptInfo {
    toss_coin: true,
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Special Summon procedure
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(lua_fn(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(lua_fn(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Special limitation
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e2, EFFECT_SPSUMMON_CONDITION);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Coin registration
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_COIN)
    effect::set_category(d, e3, CATEGORY_COIN);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // e3:SetTarget(s.cointg)
    effect::set_target(d, e3, Some(cost_target(cointg)));
    // e3:SetOperation(s.coinop)
    effect::set_operation(d, e3, Some(operation(coinop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // aux.DoubleSnareValidity(c,LOCATION_MZONE)
    aux::double_snare_validity(d, Some(c), LOCATION_MZONE, None);
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    if a.card(1).is_none() {
        return true;
    }
    // local tp=e:GetHandlerPlayer()
    let e = a.e(0);
    let tp = effect::get_handler_player(d, e);
    // local rg=Duel.GetMatchingGroup(Card.IsAbleToGraveAsCost,tp,LOCATION_MZONE,0,nil)
    let rg = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_grave_as_cost(d, c)), tp, LOCATION_MZONE, 0, NO_EX);
    // return Duel.GetLocationCount(tp,LOCATION_MZONE)>-3 and #rg>2 and aux.SelectUnselectGroup(rg,e,tp,3,3,nil,0)
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > -3
        && rg.len() > 2
        && matches!(
            aux::select_unselect_group(d, &rg, Some(e), tp, Some(3), Some(3), None, 0, tp, None, None, None, false),
            Err(true)
        )
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,c)`
fn sptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    // local rg=Duel.GetMatchingGroup(Card.IsAbleToGraveAsCost,tp,LOCATION_MZONE,0,nil)
    let rg = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_grave_as_cost(d, c)), tp, LOCATION_MZONE, 0, NO_EX);
    // local g=aux.SelectUnselectGroup(rg,e,tp,3,3,nil,1,tp,HINTMSG_TOGRAVE,nil,nil,true)
    let g = aux::select_unselect_group(d, &rg, Some(e), tp, Some(3), Some(3), None, 1, tp, Some(HINTMSG_TOGRAVE as u64), None, None, true)
        .unwrap_or_default();
    // if #g>0 then
    if !g.is_empty() {
        // g:KeepAlive()
        // e:SetLabelObject(g)
        effect::set_label_object(d, e, Value::Group(g));
        // return true
        return true;
    }
    // return false
    false
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp,c)`
fn spop(d: &mut Duel, a: Args) {
    let e = a.e(0);
    // local g=e:GetLabelObject()
    let g = effect::get_label_object(d, e).as_group().cloned();
    // if not g or #g~=3 then return end
    let g = match g {
        Some(g) if g.len() == 3 => g,
        _ => return,
    };
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    // g:DeleteGroup()
}
/// `s.cointg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cointg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_COIN,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_COIN, None, 0, a.tp, 1);
    true
}
/// `s.coinop(e,tp,eg,ep,ev,re,r,rp)`
fn coinop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) or c:IsFacedown() then return end
    if !card::is_relate_to_effect(d, c, a.e) || card::is_facedown(d, c) {
        return;
    }
    // s.arcanareg(c,Arcana.TossCoin(c,tp))
    let coin = aux::arcana::toss_coin(d, c, a.tp);
    arcanareg(d, c, coin);
}
/// `s.arcanareg(c,coin)`
fn arcanareg(d: &mut Duel, c: CardId, coin: i32) {
    // coin effect
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_BATTLE_DESTROYING)
    effect::set_code(d, e1, EVENT_BATTLE_DESTROYING);
    // e1:SetCondition(s.thcon)
    effect::set_condition(d, e1, Some(condition(thcon)));
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e2, aux::stringid(ID, 2));
    // e2:SetCategory(CATEGORY_NEGATE+CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_NEGATE + CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_QUICK_F)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_F);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DAMAGE_CAL)
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(s.negcon)
    effect::set_condition(d, e2, Some(condition(negcon)));
    // e2:SetTarget(s.negtg)
    effect::set_target(d, e2, Some(cost_target(negtg)));
    // e2:SetOperation(s.negop)
    effect::set_operation(d, e2, Some(operation(negop)));
    // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Arcana.RegisterCoinResult(c,coin)
    aux::arcana::register_coin_result(d, c, coin);
}
/// `s.thcon(e,tp,eg,ep,ev,re,r,rp)`
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return Arcana.GetCoinResult(c)==COIN_HEADS and c:IsRelateToBattle() and c:GetBattleTarget():IsLocation(LOCATION_GRAVE)
    if aux::arcana::get_coin_result(d, c) != Some(COIN_HEADS as i64) {
        return false;
    }
    if !card::is_relate_to_battle(d, c) {
        return false;
    }
    let bt = card::get_battle_target(d, c).nil();
    card::is_location(d, bt, LOCATION_GRAVE)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and chkc:IsAbleToHand() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && card::is_able_to_hand(d, chkc, None);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsAbleToHand,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_able_to_hand(d, c, None)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectTarget(tp,Card.IsAbleToHand,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&|d, c| card::is_able_to_hand(d, c, None)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, g, 1, 0, 0);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.SendtoHand(tc,nil,REASON_EFFECT)
        duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,tc)
        duel::confirm_cards(d, 1 - a.tp, tc);
    }
}
/// `s.negcon(e,tp,eg,ep,ev,re,r,rp)`
fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    let re = a.re.nil();
    // if not re:IsHasProperty(EFFECT_FLAG_CARD_TARGET) then return false end
    if !effect::is_has_property(d, re, EFFECT_FLAG_CARD_TARGET, 0) {
        return false;
    }
    // local g=Duel.GetChainInfo(ev,CHAININFO_TARGET_CARDS)
    let info = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS]);
    // if not g or not g:IsContains(c) then return false end
    match info[0].as_group() {
        Some(g) if g.contains(c) => {}
        _ => return false,
    }
    // return Arcana.GetCoinResult(c)==COIN_TAILS and (re:IsHasType(EFFECT_TYPE_ACTIVATE) or re:IsMonsterEffect())
    aux::arcana::get_coin_result(d, c) == Some(COIN_TAILS as i64)
        && (effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) || effect::is_monster_effect(d, re))
}
/// `s.negtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:GetFlagEffect(id)==0 end
    if a.chk == Some(0) {
        return card::get_flag_effect(d, c, ID) == 0;
    }
    // if c:IsHasEffect(EFFECT_REVERSE_UPDATE) then
    if !card::is_has_effect(d, c, EFFECT_REVERSE_UPDATE, None).is_empty() {
        // c:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1)
        card::register_flag_effect(d, c, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    // if re:GetHandler():IsRelateToEffect(re) and re:GetHandler():IsDestructable() then
    let re = a.re.nil();
    let rh = effect::get_handler(d, re).nil();
    if card::is_relate_to_effect(d, rh, re) && card::is_destructable(d, rh, None) {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, 0, 0);
    }
    true
}
/// `s.negop(e,tp,eg,ep,ev,re,r,rp)`
fn negop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsFaceup() or c:GetAttack()<1000 or not c:IsRelateToEffect(e) or Duel.GetCurrentChain()~=ev+1 then
    //     return
    if !card::is_faceup(d, c)
        || card::get_attack(d, c) < 1000
        || !card::is_relate_to_effect(d, c, a.e)
        || duel::get_current_chain(d, false) != a.ev as i32 + 1
    {
        return;
    }
    // if Duel.NegateActivation(ev) then
    if duel::negate_activation(d, a.ev as u8) {
        // if re:GetHandler():IsRelateToEffect(re) then
        let re = a.re.nil();
        let rh = effect::get_handler(d, re).nil();
        if card::is_relate_to_effect(d, rh, re) {
            // Duel.Destroy(re:GetHandler(),REASON_EFFECT)
            duel::destroy(d, rh, REASON_EFFECT, None, None);
        }
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_COPY_INHERIT)
        effect::set_property(d, e1, EFFECT_FLAG_COPY_INHERIT, 0);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(-1000)
        effect::set_value(d, e1, -1000);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
