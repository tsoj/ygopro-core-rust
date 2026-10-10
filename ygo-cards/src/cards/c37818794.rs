//! Red-Eyes Dark Dragoon
use super::prelude::*;

pub const ID: u32 = 37818794;
pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_DARK_MAGICIAN, CARD_REDEYES_B_DRAGON],
    material: &[CARD_DARK_MAGICIAN, CARD_REDEYES_B_DRAGON],
    material_setcode: &[SET_RED_EYES, SET_DARK_MAGICIAN],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,CARD_DARK_MAGICIAN,{CARD_REDEYES_B_DRAGON,s.ffilter})
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::Code(CARD_DARK_MAGICIAN),
            fusion::Mat::Any(vec![fusion::Mat::Code(CARD_REDEYES_B_DRAGON), fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| ffilter(d, c, fc, st, tp)))]),
        ],
    );
    // register effect
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_SINGLE)
    effect::set_type(d, e0, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_SINGLE);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e0:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e0, EVENT_SPSUMMON_SUCCESS);
    // e0:SetCondition(s.regcon)
    effect::set_condition(d, e0, Some(condition(regcon)));
    // e0:SetOperation(s.regop)
    effect::set_operation(d, e0, Some(operation(regop)));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // material count check
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_MATERIAL_CHECK)
    effect::set_code(d, e1, EFFECT_MATERIAL_CHECK);
    // e1:SetValue(s.valcheck)
    effect::set_value_fn(d, e1, lua_fn(valcheck));
    // e1:SetLabelObject(e0)
    effect::set_label_object(d, e1, Value::Effect(e0));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Immune
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e2, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetValue(1)
    effect::set_value(d, e2, 1);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e3, EFFECT_INDESTRUCTABLE_EFFECT);
    // e3:SetValue(1)
    effect::set_value(d, e3, 1);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // negate
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_NEGATE+CATEGORY_DESTROY)
    effect::set_category(d, e4, CATEGORY_NEGATE + CATEGORY_DESTROY);
    // e4:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e4, EFFECT_TYPE_QUICK_O);
    // e4:SetCode(EVENT_CHAINING)
    effect::set_code(d, e4, EVENT_CHAINING);
    // e4:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DAMAGE_CAL)
    effect::set_property(d, e4, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCountLimit(1)
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    // e4:SetCondition(s.discon)
    effect::set_condition(d, e4, Some(condition(discon)));
    // e4:SetCost(s.discost)
    effect::set_cost(d, e4, Some(cost_target(discost)));
    // e4:SetTarget(s.distg)
    effect::set_target(d, e4, Some(cost_target(distg)));
    // e4:SetOperation(s.disop)
    effect::set_operation(d, e4, Some(operation(disop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.ffilter(c,fc,sumtype,tp)`
fn ffilter(d: &mut Duel, c: CardId, fc: CardId, sumtype: u64, tp: u8) -> bool {
    // return c:IsRace(RACE_DRAGON,fc,sumtype,tp) and c:IsType(TYPE_EFFECT,fc,sumtype,tp)
    card::is_race_ctx(d, c, RACE_DRAGON as u64, Some(fc), sumtype, Some(tp)) && card::is_type_ctx(d, c, TYPE_EFFECT, Some(fc), sumtype, Some(tp))
}
/// `s.valcheck(e,c)`
fn valcheck(d: &mut Duel, a: Args) -> Value {
    let (e, c) = (a.e(0), a.c(1));
    // local g=c:GetMaterial()
    let g = card::get_material(d, c);
    // local ct=g:FilterCount(Card.IsType,nil,TYPE_NORMAL)
    let ct = group::filter_count(d, &g, &|d, x| card::is_type(d, x, TYPE_NORMAL), NO_EX);
    // e:GetLabelObject():SetLabel(ct)
    let lo = effect::get_label_object(d, e).as_effect().nil();
    effect::set_label(d, lo, &[ct as i64]);
    Value::Nil
}
/// `s.regcon(e,tp,eg,ep,ev,re,r,rp)`
fn regcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsFusionSummoned() and e:GetLabel()>0
    card::is_fusion_summoned(d, c) && effect::get_label(d, a.e) > 0
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ct=e:GetLabel()
    let ct = effect::get_label(d, a.e);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_DAMAGE)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_DAMAGE);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetReset(RESET_EVENT|(RESETS_STANDARD&~RESET_TURN_SET))
    effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD & !RESET_TURN_SET), 1);
    // e1:SetCountLimit(ct)
    effect::set_count_limit(d, e1, ct as u8, 0, 0, 0);
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(Card.IsDestructable,tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_destructable(d, c, None)), tp, 0, LOCATION_MZONE, NO_EX);
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectMatchingCard(tp,Card.IsDestructable,tp,0,LOCATION_MZONE,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| card::is_destructable(d, c, None)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.HintSelection(g)
        duel::hint_selection(d, g.clone(), true);
        // local atk=g:GetFirst():GetTextAttack()
        let tc = group::get_first(d, &g).nil();
        let mut atk = card::get_text_attack(d, tc) as i64;
        // if atk<0 then atk=0 end
        if atk < 0 {
            atk = 0;
        }
        // if Duel.Destroy(g:GetFirst(),REASON_EFFECT)~=0 then
        if duel::destroy(d, CardSet::from_card(tc), REASON_EFFECT, None, None) != 0 {
            // Duel.Damage(1-tp,atk,REASON_EFFECT)
            duel::damage(d, 1 - tp, atk, REASON_EFFECT, false, None);
        }
    }
}
/// `s.discon(e,tp,eg,ep,ev,re,r,rp)`
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsStatus(STATUS_BATTLE_DESTROYED) then return false end
    if card::is_status(d, c, STATUS_BATTLE_DESTROYED) {
        return false;
    }
    // return Duel.IsChainNegatable(ev)
    duel::is_chain_negatable(d, a.ev as u8)
}
/// `s.discard`: `Card.IsDiscardable` as a filter
fn discardable(d: &mut Duel, c: CardId) -> bool {
    card::is_discardable(d, c, None, None)
}
/// `s.discost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn discost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsDiscardable,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&discardable), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.DiscardHand(tp,Card.IsDiscardable,1,1,REASON_COST|REASON_DISCARD)
    duel::discard_hand(d, a.tp, Some(&discardable), 1, 1, REASON_COST | REASON_DISCARD, NO_EX);
    true
}
/// `s.distg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    // if re:GetHandler():IsRelateToEffect(re) then
    let re = a.re.nil();
    let rh = effect::get_handler(d, re).nil();
    if card::is_relate_to_effect(d, rh, re) {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, 0, 0);
    }
    true
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    let re = a.re.nil();
    // if Duel.NegateActivation(ev) and re:GetHandler():IsRelateToEffect(re) and Duel.Destroy(eg,REASON_EFFECT)~=0
    //   and e:GetHandler():IsRelateToEffect(e) and e:GetHandler():IsFaceup() then
    if !duel::negate_activation(d, a.ev as u8) {
        return;
    }
    let rh = effect::get_handler(d, re).nil();
    if !card::is_relate_to_effect(d, rh, re) {
        return;
    }
    if duel::destroy(d, a.eg.clone().nil(), REASON_EFFECT, None, None) == 0 {
        return;
    }
    let h = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, h, a.e) && card::is_faceup(d, h) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(1000)
        effect::set_value(d, e1, 1000);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
        // e:GetHandler():RegisterEffect(e1)
        card::register_effect(d, h, e1, false);
    }
}
