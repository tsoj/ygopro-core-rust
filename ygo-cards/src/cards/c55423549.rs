//! Leo Wizard the Dark Fiend
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 55423549;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Monsters whose original Level is 4 or lower with more than 1350 ATK and/or DEF require 1 Tribute to Normal Summon from the hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e1:SetCode(EFFECT_LIMIT_SUMMON_PROC)
    effect::set_code(d, e1, EFFECT_LIMIT_SUMMON_PROC);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_HAND,LOCATION_HAND)
    effect::set_target_range(d, e1, LOCATION_HAND, LOCATION_HAND);
    // e1:SetCondition(s.nsumcon)
    effect::set_condition(d, e1, Some(lua_fn(nsumcon)));
    // e1:SetTarget(aux.FieldSummonProcTg(s.nsumtg,s.nsumcost))
    effect::set_target(d, e1, Some(aux::field_summon_proc_tg(Some(effect_card_fn(nsumtg)), Some(lua_fn(nsumcost)))));
    // e1:SetOperation(s.nsumop)
    effect::set_operation(d, e1, Some(lua_fn(nsumop)));
    // e1:SetValue(SUMMON_TYPE_TRIBUTE)
    effect::set_value(d, e1, SUMMON_TYPE_TRIBUTE as i64);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Destroy a LIGHT monster that this card is battling
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_BATTLE_START)
    effect::set_code(d, e2, EVENT_BATTLE_START);
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Add 1 monster with 1350 ATK from your Deck to your hand
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e3, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e3, EVENT_DESTROYED);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetCondition(function(e) return e:GetHandler():IsReason(REASON_BATTLE|REASON_EFFECT) end)
    effect::set_condition(d, e3, Some(condition(cond3)));
    // e3:SetTarget(s.thtg)
    effect::set_target(d, e3, Some(cost_target(thtg)));
    // e3:SetOperation(s.thop)
    effect::set_operation(d, e3, Some(operation(thop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.nsumcon(e,c,minc)`
fn nsumcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // local rg=Duel.GetReleaseGroup(tp)
    let rg = duel::get_release_group(d, tp, false, false, None);
    // return aux.SelectUnselectGroup(rg,e,tp,1,1,aux.ChkfMMZ(1),0)
    matches!(
        aux::select_unselect_group(d, &rg, Some(a.e(0)), tp, Some(1), Some(1), Some(&aux::chkf_mmz(1)), 0, tp, None, None, None, false),
        Err(true)
    )
}
/// `s.nsumtg(e,c)`
fn nsumtg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:GetOriginalLevel()<=4 and (c:GetAttack()>1350 or c:GetDefense()>1350)
    card::get_original_level(d, c) <= 4 && (card::get_attack(d, c) > 1350 || card::get_defense(d, c) > 1350)
}
/// `s.nsumcost(e,tp,eg,ep,ev,re,r,rp,chk,c)`
fn nsumcost(d: &mut Duel, a: Args) -> bool {
    let (e, tp) = (a.e(0), a.p(1));
    // local rg=Duel.GetReleaseGroup(tp)
    let rg = duel::get_release_group(d, tp, false, false, None);
    // local g=aux.SelectUnselectGroup(rg,e,tp,1,1,aux.ChkfMMZ(1),1,tp,HINTMSG_RELEASE,nil,nil,true)
    let g = aux::select_unselect_group(d, &rg, Some(e), tp, Some(1), Some(1), Some(&aux::chkf_mmz(1)), 1, tp, Some(HINTMSG_RELEASE as u64), None, None, true)
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
/// `s.nsumop(e,tp,eg,ep,ev,re,r,rp,c)`
fn nsumop(d: &mut Duel, a: Args) -> Value {
    let e = a.e(0);
    // local g=e:GetLabelObject()
    // if not g then return end
    let Some(g) = effect::get_label_object(d, e).as_group().cloned() else {
        return Value::Nil;
    };
    // c:SetMaterial(g)
    let c = a.c(8);
    card::set_material(d, c, g.clone());
    // Duel.Release(g,REASON_COST|REASON_SUMMON|REASON_MATERIAL)
    duel::release(d, g, REASON_COST | REASON_SUMMON | REASON_MATERIAL, None);
    // g:DeleteGroup()
    Value::Nil
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // local bc=e:GetHandler():GetBattleTarget()
    let h = effect::get_handler(d, a.e).nil();
    let bc = card::get_battle_target(d, h);
    // if chk==0 then return bc and bc:IsFaceup() and bc:IsAttribute(ATTRIBUTE_LIGHT) end
    if a.chk == Some(0) {
        return match bc {
            None => false,
            Some(bc) => card::is_faceup(d, bc) && card::is_attribute(d, bc, ATTRIBUTE_LIGHT),
        };
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,bc,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, bc.map(CardSet::from_card), 1, a.tp, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local bc=e:GetHandler():GetBattleTarget()
    let h = effect::get_handler(d, a.e).nil();
    // if bc and bc:IsRelateToBattle() then
    if let Some(bc) = card::get_battle_target(d, h) {
        if card::is_relate_to_battle(d, bc) {
            // Duel.Destroy(bc,REASON_EFFECT)
            duel::destroy(d, bc, REASON_EFFECT, None, None);
        }
    }
}
/// `function(e) return e:GetHandler():IsReason(REASON_BATTLE|REASON_EFFECT) end`
fn cond3(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_reason(d, h, REASON_BATTLE | REASON_EFFECT)
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttack(1350) and c:IsAbleToHand()
    card::is_attack(d, c, &[1350]) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
    }
}
