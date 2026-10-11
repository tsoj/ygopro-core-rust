//! Azamina
//! 贖罪神女
use super::prelude::*;

pub const ID: u32 = 17749468;
pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[65033975, 85065943],
    material_setcode: &[SET_AZAMINA],
    miracle_synchro_fusion: true,
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Materials: 1 "Queen Azamina" + 1 Fusion or Synchro Monster
    // Fusion.AddProcMix(c,true,true,65033975,aux.FilterBoolFunctionEx(Card.IsType,TYPE_FUSION|TYPE_SYNCHRO))
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::Code(65033975),
            fusion::Mat::F(fusion::fus_mat(|d, cc, fc, st, tp| card::is_type_ctx(d, cc, TYPE_FUSION | TYPE_SYNCHRO, Some(fc), st, Some(tp)))),
        ],
    );
    // c:AddMustFirstBeFusionSummoned()
    card::add_must_first_be_fusion_summoned(d, c);
    // Special Summon this card (from your Extra Deck) by Tributing 1 "Saint Azamina" you control and 1 face-up monster your opponent controls
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e0, aux::stringid(ID, 0));
    // e0:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e0, EFFECT_TYPE_FIELD);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e0:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e0, EFFECT_SPSUMMON_PROC);
    // e0:SetRange(LOCATION_EXTRA)
    effect::set_range(d, e0, LOCATION_EXTRA);
    // e0:SetCondition(s.selfspcon)
    effect::set_condition(d, e0, Some(lua_fn(selfspcon)));
    // e0:SetTarget(s.selfsptg)
    effect::set_target(d, e0, Some(lua_fn(selfsptg)));
    // e0:SetOperation(s.selfspop)
    effect::set_operation(d, e0, Some(lua_fn(selfspop)));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Cannot be destroyed by card effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_EFFECT);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Each time your opponent activates a card or effect, all monsters your opponent currently controls lose 500 ATK immediately after it resolves
    // local e2a=Effect.CreateEffect(c)
    let e2a = effect::create_effect(d, c);
    // e2a:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2a, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2a:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e2a, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e2a:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2a, EVENT_CHAINING);
    // e2a:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2a, LOCATION_MZONE);
    // e2a:SetOperation(aux.chainreg)
    effect::set_operation(d, e2a, Some(aux::chainreg()));
    // c:RegisterEffect(e2a)
    card::register_effect(d, c, e2a, false);
    // local e2b=Effect.CreateEffect(c)
    let e2b = effect::create_effect(d, c);
    // e2b:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2b, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2b:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2b, EFFECT_FLAG_DELAY, 0);
    // e2b:SetCode(EVENT_CHAIN_SOLVED)
    effect::set_code(d, e2b, EVENT_CHAIN_SOLVED);
    // e2b:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2b, LOCATION_MZONE);
    // e2b:SetCondition(function(e) return e:GetHandler():HasFlagEffect(1) end)
    effect::set_condition(d, e2b, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        card::has_flag_effect(d, h, 1, None)
    })));
    // e2b:SetOperation(s.atkop)
    effect::set_operation(d, e2b, Some(operation(atkop)));
    // c:RegisterEffect(e2b)
    card::register_effect(d, c, e2b, false);
    // Your opponent cannot activate the effects of monsters with 0 ATK
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e3:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e3, EFFECT_CANNOT_ACTIVATE);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetTargetRange(0,1)
    effect::set_target_range(d, e3, 0, 1);
    // e3:SetValue(function(e,re,tp) return re:IsMonsterEffect() and re:GetHandler():IsAttack(0) end)
    effect::set_value_fn(d, e3, lua_fn(cannot_activate_value));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `function(e,re,tp) return re:IsMonsterEffect() and re:GetHandler():IsAttack(0) end`
fn cannot_activate_value(d: &mut Duel, a: Args) -> bool {
    let re = a.e(1);
    if !effect::is_monster_effect(d, re) {
        return false;
    }
    let h = effect::get_handler(d, re).nil();
    card::is_attack(d, h, &[0])
}
/// `s.selfspcostfilter(c,tp,fc)`
fn selfspcostfilter(d: &mut Duel, c: CardId, tp: u8, fc: Option<CardId>) -> bool {
    // return ((c:IsCode(85065943) or c:IsSummonCode(fc,MATERIAL_FUSION,tp,85065943)) or c:IsControler(1-tp)) and c:IsReleasable()
    //     and c:IsCanBeFusionMaterial(fc,MATERIAL_FUSION) and (c:IsControler(tp) or c:IsFaceup())
    (card::is_code(d, c, &[85065943])
        || card::is_summon_code(d, c, fc, MATERIAL_FUSION as u64, tp, &[85065943])
        || card::is_controler(d, c, 1 - tp))
        && card::is_releasable(d, c, None, None)
        && card::is_can_be_fusion_material(d, c, fc, Some(MATERIAL_FUSION as u64), None)
        && (card::is_controler(d, c, tp) || card::is_faceup(d, c))
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return Duel.GetLocationCountFromEx(tp,tp,sg,e:GetHandler())>0 and sg:FilterCount(Card.IsControler,nil,tp)==1
    let h = effect::get_handler(d, e.nil()).nil();
    let res = duel::get_location_count_from_ex(d, tp, Some(tp), sg.into(), duel::ExtraCard::Card(h), None).0 > 0
        && group::filter_count(d, sg, &|d, c| card::is_controler(d, c, tp), NO_EX) == 1;
    (res, false)
}
/// `s.selfspcon(e,c)`
fn selfspcon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // local mg=Duel.GetMatchingGroup(s.selfspcostfilter,tp,LOCATION_MZONE,LOCATION_MZONE,nil,tp,c)
    let mg = duel::get_matching_group(d, Some(&|d, x| selfspcostfilter(d, x, tp, Some(c))), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // return #mg>=2 and aux.SelectUnselectGroup(mg,e,tp,2,2,s.rescon,0)
    if mg.len() < 2 {
        return false;
    }
    let rs: aux::Rescon = &rescon;
    matches!(aux::select_unselect_group(d, &mg, Some(e), tp, Some(2), Some(2), Some(rs), 0, tp, None, None, None, false), Err(true))
}
/// `s.selfsptg(e,tp,eg,ep,ev,re,r,rp,chk,c)`
fn selfsptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    let c = a.card(9);
    // local mg=Duel.GetMatchingGroup(s.selfspcostfilter,tp,LOCATION_MZONE,LOCATION_MZONE,nil,tp,c)
    let mg = duel::get_matching_group(d, Some(&|d, x| selfspcostfilter(d, x, tp, c)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // local g=aux.SelectUnselectGroup(mg,e,tp,2,2,s.rescon,1,tp,HINTMSG_RELEASE,nil,nil,true)
    let rs: aux::Rescon = &rescon;
    let g = aux::select_unselect_group(d, &mg, Some(e), tp, Some(2), Some(2), Some(rs), 1, tp, Some(HINTMSG_RELEASE as u64), None, None, true).unwrap_or_default();
    // if #g>0 then
    if !g.is_empty() {
        // e:SetLabelObject(g)
        effect::set_label_object(d, e, Value::Group(g));
        // return true
        return true;
    }
    // return false
    false
}
/// `s.selfspop(e,tp,eg,ep,ev,re,r,rp,c)`
fn selfspop(d: &mut Duel, a: Args) {
    let e = a.e(0);
    // local g=e:GetLabelObject()
    // if not g then return end
    // Duel.Release(g,REASON_COST|REASON_MATERIAL)
    if let Value::Group(g) = effect::get_label_object(d, e) {
        duel::release(d, g, REASON_COST | REASON_MATERIAL, None);
    }
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // if rp~=1-tp then return end
    if a.rp != 1 - a.tp {
        return;
    }
    // local g=Duel.GetMatchingGroup(Card.IsFaceup,tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, 0, LOCATION_MZONE, NO_EX);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // Duel.Hint(HINT_CARD,0,id)
    duel::hint(d, HINT_CARD as u8, 0, ID as u64);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // for tc in g:Iter() do
    for tc in group::iter(d, &g) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(-500)
        effect::set_value(d, e1, -500);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
