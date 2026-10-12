//! Salamangreat Shrine
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 101403062;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[1295111], listed_series: &[SET_SALAMANGREAT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // "Salamangreat" monsters you control are unaffected by your opponent's activated FIRE monsters' effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_IMMUNE_EFFECT)
    effect::set_code(d, e1, EFFECT_IMMUNE_EFFECT);
    // e1:SetRange(LOCATION_FZONE)
    effect::set_range(d, e1, LOCATION_FZONE);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetTarget(function(e,c) return c:IsSetCard(SET_SALAMANGREAT) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_SALAMANGREAT as u16]))));
    // e1:SetValue(function(e,te) ... end)
    effect::set_value_fn(d, e1, lua_fn(immune_value));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If you Link Summon a "Salamangreat" Link Monster, you can use 1 "Salamangreat" Link Monster you control with its same name as the entire material
    // local e2a=Effect.CreateEffect(c)
    let e2a = effect::create_effect(d, c);
    // e2a:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2a, aux::stringid(ID, 0));
    // e2a:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2a, EFFECT_TYPE_FIELD);
    // e2a:SetProperty(EFFECT_FLAG_UNCOPYABLE+EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e2a, EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e2a:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e2a, EFFECT_SPSUMMON_PROC);
    // e2a:SetRange(LOCATION_EXTRA)
    effect::set_range(d, e2a, LOCATION_EXTRA);
    // e2a:SetCondition(s.reinclinkcon)
    effect::set_condition(d, e2a, Some(lua_fn(reinclinkcon)));
    // e2a:SetTarget(s.reinclinktg)
    effect::set_target(d, e2a, Some(lua_fn(reinclinktg)));
    // e2a:SetOperation(s.reinclinkop)
    effect::set_operation(d, e2a, Some(lua_fn(reinclinkop)));
    // e2a:SetValue(SUMMON_TYPE_LINK)
    effect::set_value(d, e2a, SUMMON_TYPE_LINK as i64);
    // local e2b=Effect.CreateEffect(c)
    let e2b = effect::create_effect(d, c);
    // e2b:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_GRANT)
    effect::set_type(d, e2b, EFFECT_TYPE_FIELD + EFFECT_TYPE_GRANT);
    // e2b:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2b, LOCATION_FZONE);
    // e2b:SetTargetRange(LOCATION_EXTRA,0)
    effect::set_target_range(d, e2b, LOCATION_EXTRA, 0);
    // e2b:SetTarget(function(e,c) return c:IsSetCard(SET_SALAMANGREAT) and c:IsLinkMonster() end)
    effect::set_target(d, e2b, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_SALAMANGREAT as u16]) && card::is_link_monster(d, c))));
    // e2b:SetLabelObject(e2a)
    effect::set_label_object(d, e2b, Value::Effect(e2a));
    // c:RegisterEffect(e2b)
    card::register_effect(d, c, e2b, false);
    // During your Main Phase: You can shuffle any number of "Salamangreat" cards from your hand into the Deck, then draw the same number of cards
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_TODECK+CATEGORY_DRAW)
    effect::set_category(d, e3, CATEGORY_TODECK + CATEGORY_DRAW);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_FZONE)
    effect::set_range(d, e3, LOCATION_FZONE);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetTarget(s.tdtg)
    effect::set_target(d, e3, Some(cost_target(tdtg)));
    // e3:SetOperation(s.tdop)
    effect::set_operation(d, e3, Some(operation(tdop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `function(e,te) return te:GetOwnerPlayer()~=e:GetHandlerPlayer() and te:IsActivated() and te:IsMonsterEffect() and te:IsCardAttribute(ATTRIBUTE_FIRE) end`
fn immune_value(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let te = a.e(1);
    effect::get_owner_player(d, te) != effect::get_handler_player(d, e)
        && effect::is_activated(d, te)
        && effect::is_monster_effect(d, te)
        && effect::is_card_attribute(d, te, ATTRIBUTE_FIRE)
}
/// `s.reincmatfilter(c,lc,tp)`
fn reincmatfilter(d: &mut Duel, c: CardId, lc: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsLinkMonster()
    //     and c:IsSummonCode(lc,SUMMON_TYPE_LINK,tp,lc:GetCode()) and c:IsCanBeLinkMaterial(lc,tp)
    //     and Duel.GetLocationCountFromEx(tp,tp,c,lc)>0
    let code = card::get_code(d, lc)[0];
    card::is_faceup(d, c)
        && card::is_link_monster(d, c)
        && card::is_summon_code(d, c, Some(lc), SUMMON_TYPE_LINK as u64, tp, &[code])
        && card::is_can_be_link_material(d, c, Some(lc), Some(tp))
        && duel::get_location_count_from_ex(d, tp, Some(tp), c.into(), duel::ExtraCard::Card(lc), None).0 > 0
}
/// `s.reinclinkcon(e,c,must,g,min,max)`
fn reinclinkcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else { return true };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // local g=Duel.GetMatchingGroup(s.reincmatfilter,tp,LOCATION_MZONE,0,nil,c,tp)
    let g = duel::get_matching_group(d, Some(&|d, x| reincmatfilter(d, x, c, tp)), tp, LOCATION_MZONE, 0, NO_EX);
    // local mustg=Auxiliary.GetMustBeMaterialGroup(tp,g,tp,c,g,REASON_LINK)
    let mut mustg = aux::get_must_be_material_group_opt(d, tp, Some(&g), tp, Some(c), Some(&g), REASON_LINK);
    // if must then mustg:Merge(must) end
    if let Some(must) = a.v(2).as_group().cloned() {
        group::add_card(&mut mustg, &must);
    }
    // return ((#mustg==1 and s.reincmatfilter(mustg:GetFirst(),c,tp)) or (#mustg==0 and #g>0))
    //     and not Duel.HasFlagEffect(tp,id)
    let first_ok = mustg.len() == 1 && {
        let f = group::get_first(d, &mustg).nil();
        reincmatfilter(d, f, c, tp)
    };
    (first_ok || (mustg.is_empty() && !g.is_empty())) && !duel::has_flag_effect(d, tp, ID, None)
}
/// `s.reinclinktg(e,tp,eg,ep,ev,re,r,rp,chk,c,must,g,min,max)`
fn reinclinktg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    let c = a.c(9);
    let must = a.v(10).as_group().cloned();
    // local g=Duel.GetMatchingGroup(s.reincmatfilter,tp,LOCATION_MZONE,0,nil,c,tp)
    let g = duel::get_matching_group(d, Some(&|d, x| reincmatfilter(d, x, c, tp)), tp, LOCATION_MZONE, 0, NO_EX);
    // local mustg=Auxiliary.GetMustBeMaterialGroup(tp,g,tp,c,g,REASON_LINK)
    let mut mustg = aux::get_must_be_material_group_opt(d, tp, Some(&g), tp, Some(c), Some(&g), REASON_LINK);
    // if must then mustg:Merge(must) end
    if let Some(must) = must {
        group::add_card(&mut mustg, &must);
    }
    // if #mustg>0 then
    if !mustg.is_empty() {
        // if #mustg>1 then return false end
        if mustg.len() > 1 {
            return false;
        }
        // e:SetLabelObject(mustg)
        effect::set_label_object(d, e, Value::Group(mustg));
        // return true
        return true;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_LMATERIAL)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_LMATERIAL as u64);
    // local tc=g:SelectUnselect(Group.CreateGroup(),tp,false,true)
    let tc = group::select_unselect(d, &g, Some(&CardSet::new()), tp, false, true, None, None);
    // if tc then
    if let Some(tc) = tc {
        // local sg=Group.FromCards(tc)
        let sg = CardSet::from_card(tc);
        // e:SetLabelObject(sg)
        effect::set_label_object(d, e, Value::Group(sg));
        // return true
        true
    // else return false end
    } else {
        false
    }
}
/// `s.reinclinkop(e,tp,eg,ep,ev,re,r,rp,c,must,g,min,max)`
fn reinclinkop(d: &mut Duel, a: Args) {
    let e = a.e(0);
    let tp = a.p(1);
    let c = a.c(8);
    // Duel.Hint(HINT_CARD,0,id)
    duel::hint(d, HINT_CARD as u8, 0, ID as u64);
    // local mg=e:GetLabelObject()
    let mg = effect::get_label_object(d, e).as_group().cloned().nil();
    // c:SetMaterial(mg)
    card::set_material(d, c, mg.clone());
    // Duel.SendtoGrave(mg,REASON_MATERIAL|REASON_LINK)
    duel::send_to_grave(d, mg, REASON_MATERIAL | REASON_LINK, None, None);
    // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
    duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
}
/// `s.tdfilter(c)`
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_SALAMANGREAT) and c:IsAbleToDeck()
    card::is_set_card(d, c, &[SET_SALAMANGREAT as u16]) && card::is_able_to_deck(d, c)
}
/// `s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanDraw(tp)
    //     and Duel.IsExistingMatchingCard(s.tdfilter,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, a.tp, 0) && duel::is_existing_matching_card(d, Some(&tdfilter), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, a.tp, LOCATION_HAND as i32);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    true
}
/// `s.tdop(e,tp,eg,ep,ev,re,r,rp)`
fn tdop(d: &mut Duel, a: &EffArgs) {
    // local hg=Duel.GetMatchingGroup(s.tdfilter,tp,LOCATION_HAND,0,nil)
    let hg = duel::get_matching_group(d, Some(&tdfilter), a.tp, LOCATION_HAND, 0, NO_EX);
    // if #hg==0 then return end
    if hg.is_empty() {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
    // local g=hg:Select(tp,1,#hg,nil)
    let n = hg.len() as u16;
    let g = group::select(d, &hg, a.tp, 1, n, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g.clone());
        // Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
        duel::send_to_deck(d, g.clone(), None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
        // local ct=#Duel.GetOperatedGroup()
        let ct = duel::get_operated_group(d).len();
        // if ct>0 then Duel.ShuffleDeck(tp) end
        if ct > 0 {
            duel::shuffle_deck(d, a.tp);
        }
        // if ct==#g and Duel.IsPlayerCanDraw(tp) then
        if ct == g.len() && duel::is_player_can_draw(d, a.tp, 0) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Draw(tp,ct,REASON_EFFECT)
            duel::draw(d, a.tp, ct as u32, REASON_EFFECT);
        }
    }
}
