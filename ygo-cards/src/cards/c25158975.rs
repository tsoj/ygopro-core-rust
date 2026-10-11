//! Destiny HERO - Destro-Dogma
use super::prelude::*;

pub const ID: u32 = 25158975;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // c:AddMustBeSpecialSummoned()
    card::add_must_be_special_summoned(d, c);
    // Must be Special Summoned (from your hand or GY) by banishing 3 Warrior and/or DARK monsters from your GY. You can only Special Summon "Destiny HERO - Death Dogma" once per turn this way
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e0, aux::stringid(ID, 0));
    // e0:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e0, EFFECT_TYPE_FIELD);
    // e0:SetProperty(EFFECT_FLAG_UNCOPYABLE+EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e0, EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e0:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e0, EFFECT_SPSUMMON_PROC);
    // e0:SetRange(LOCATION_HAND|LOCATION_GRAVE)
    effect::set_range(d, e0, LOCATION_HAND | LOCATION_GRAVE);
    // e0:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e0, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e0:SetCondition(s.spcon)
    effect::set_condition(d, e0, Some(lua_fn(spcon)));
    // e0:SetTarget(s.sptg)
    effect::set_target(d, e0, Some(lua_fn(sptg)));
    // e0:SetOperation(s.spop)
    effect::set_operation(d, e0, Some(lua_fn(spop)));
    // e0:SetValue(1)
    effect::set_value(d, e0, 1);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // If this card is Special Summoned: You can activate this effect; inflict 2000 damage to your opponent during the next Standby Phase
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e1, CATEGORY_DAMAGE);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCondition(function(e) return e:GetHandler():IsSummonType(SUMMON_TYPE_SPECIAL+1) end)
    effect::set_condition(d, e1, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        card::is_summon_type(d, h, &[SUMMON_TYPE_SPECIAL + 1])
    })));
    // e1:SetTarget(s.damtg)
    effect::set_target(d, e1, Some(cost_target(damtg)));
    // e1:SetOperation(s.damop)
    effect::set_operation(d, e1, Some(operation(damop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Once per turn, when your opponent activates a card or effect (Quick Effect): You can Fusion Summon 1 DARK or Warrior Fusion Monster from your Extra Deck, by shuffling its materials from your hand, field, and/or GY into the Deck
    // local fusion_params={...}
    let params = fusion::SummonEff {
        // fusfilter=function(c) return c:IsAttribute(ATTRIBUTE_DARK) or c:IsRace(RACE_WARRIOR) end
        fusfilter: Some(fusion::fus_filter(|d, c, _tp| card::is_attribute(d, c, ATTRIBUTE_DARK) || card::is_race(d, c, RACE_WARRIOR))),
        // extratg=function(e,tp,eg,ep,ev,re,r,rp,chk) ... end
        extratg: Some(cost_target(extratg)),
        // extraop=Fusion.ShuffleMaterial
        extraop: Some(fusion::shuffle_material()),
        // extrafil=function(e,tp,mg) return Duel.GetMatchingGroup(Fusion.IsMonsterFilter(Card.IsAbleToDeck),tp,LOCATION_GRAVE,0,nil) end
        extrafil: Some(std::sync::Arc::new(extrafil)),
        ..Default::default()
    };
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e2, aux::stringid(ID, 2));
    // e2:SetCategory(CATEGORY_FUSION_SUMMON+CATEGORY_SPECIAL_SUMMON+CATEGORY_TODECK)
    effect::set_category(d, e2, CATEGORY_FUSION_SUMMON + CATEGORY_SPECIAL_SUMMON + CATEGORY_TODECK);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return ep==1-tp end)
    effect::set_condition(d, e2, Some(condition(|_d, a| a.ep == 1 - a.tp)));
    // e2:SetTarget(Fusion.SummonEffTG(fusion_params))
    let tg = fusion::summon_eff_tg(d, params.clone());
    effect::set_target(d, e2, Some(tg));
    // e2:SetOperation(Fusion.SummonEffOP(fusion_params))
    let op = fusion::summon_eff_op(d, params);
    effect::set_operation(d, e2, Some(op));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.spcostfilter(c)`
fn spcostfilter(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsRace(RACE_WARRIOR) or c:IsAttribute(ATTRIBUTE_DARK)) and c:IsAbleToRemoveAsCost() and aux.SpElimFilter(c,true)
    (card::is_race(d, c, RACE_WARRIOR) || card::is_attribute(d, c, ATTRIBUTE_DARK))
        && card::is_able_to_remove_as_cost(d, c, None)
        && aux::sp_elim_filter(d, c, true, false)
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, a.e(0));
    // local g=Duel.GetMatchingGroup(s.spcostfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,c)
    let g = duel::get_matching_group(d, Some(&spcostfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, c.into());
    // return #g>=3 and Duel.GetMZoneCount(tp,g)>0 and aux.SelectUnselectGroup(g,e,tp,3,3,aux.ChkfMMZ(1),0)
    if g.len() < 3 {
        return false;
    }
    if duel::get_m_zone_count(d, tp, (&g).into(), None, None, None).0 <= 0 {
        return false;
    }
    let chkf = aux::chkf_mmz(1);
    let rs: aux::Rescon = &chkf;
    matches!(aux::select_unselect_group(d, &g, Some(a.e(0)), tp, Some(3), Some(3), Some(rs), 0, tp, None, None, None, false), Err(true))
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,c)`
fn sptg(d: &mut Duel, a: Args) -> bool {
    let (e, tp) = (a.e(0), a.p(1));
    // local g=Duel.GetMatchingGroup(s.spcostfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,c)
    let g = duel::get_matching_group(d, Some(&spcostfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, a.card(9).into());
    // local sg=aux.SelectUnselectGroup(g,e,tp,3,3,aux.ChkfMMZ(1),1,tp,HINTMSG_REMOVE,nil,nil,true)
    let chkf = aux::chkf_mmz(1);
    let rs: aux::Rescon = &chkf;
    let sg = aux::select_unselect_group(d, &g, Some(e), tp, Some(3), Some(3), Some(rs), 1, tp, Some(HINTMSG_REMOVE as u64), None, None, true).unwrap_or_default();
    // if sg and #sg==3 then
    if sg.len() == 3 {
        // e:SetLabelObject(sg)
        effect::set_label_object(d, e, Value::Group(sg));
        // return true
        return true;
    }
    // return false
    false
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp,c)`
fn spop(d: &mut Duel, a: Args) {
    // local sg=e:GetLabelObject()
    let sg = effect::get_label_object(d, a.e(0)).as_group().cloned();
    // if sg and #sg==3 then
    if let Some(sg) = sg {
        if sg.len() == 3 {
            // Duel.Remove(sg,POS_FACEUP,REASON_COST)
            duel::remove(d, sg, POS_FACEUP as u8, REASON_COST, None, None);
        }
    }
}
/// `s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,2000)
    duel::set_possible_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, 2000);
    true
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // aux.RegisterClientHint(c,nil,tp,0,1,aux.Stringid(id,3),RESET_PHASE|PHASE_STANDBY)
    aux::register_client_hint(d, c, 0, tp, 0, 1, Some(aux::stringid(ID, 3)), RESET_PHASE | PHASE_STANDBY, None);
    // Inflict 2000 damage to your opponent during the next Standby Phase
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_PHASE+PHASE_STANDBY)
    effect::set_code(d, e1, EVENT_PHASE + PHASE_STANDBY);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetOperation(function() Duel.Hint(HINT_CARD,0,id) Duel.Damage(1-tp,2000,REASON_EFFECT) end)
    effect::set_operation(d, e1, Some(operation(move |d, _a| {
        duel::hint(d, HINT_CARD as u8, 0, ID as u64);
        duel::damage(d, 1 - tp, 2000, REASON_EFFECT, false, None);
    })));
    // e1:SetReset(RESET_PHASE|PHASE_STANDBY)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_STANDBY, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}
/// `s.extrafil(e,tp,mg)`
fn extrafil(d: &mut Duel, _e: EffectId, tp: u8, _mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    // return Duel.GetMatchingGroup(Fusion.IsMonsterFilter(Card.IsAbleToDeck),tp,LOCATION_GRAVE,0,nil)
    let f = fusion::is_monster_filter(fusion::fus_filter(|d, c, _tp| card::is_able_to_deck(d, c)));
    (Some(duel::get_matching_group(d, Some(&|d, c| f(d, c, tp)), tp, LOCATION_GRAVE, 0, NO_EX)), None)
}
/// `s.extratg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn extratg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,1,tp,LOCATION_HAND|LOCATION_MZONE|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, a.tp, (LOCATION_HAND | LOCATION_MZONE | LOCATION_GRAVE) as i32);
    true
}
